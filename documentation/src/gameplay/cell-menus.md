# Cell menus, audiences, and crafting tables

Cell menus generalize grid-based UI beyond the player's personal inventory.

Examples:

- crafting tables;
- chests;
- machines;
- shops;
- temporary trade windows.

They reuse `Inventory`, `InventoryLayout`, `InventoryCell`, and `ItemInstance`.

## Cell menu state

```rust
pub struct CellMenuState {
    pub id: CellMenuId,
    pub title: String,
    pub audience: Audience,
    pub inventory: Inventory,
}
```

`CellMenuId` identifies one state object. For a block-anchored menu it should
include world identity and block position.

## Audience

Current audience modes:

```rust
pub enum Audience {
    Personal(PlayerId),
    Shared(AudienceId),
    Everyone,
}
```

Personal menus allow only their owner.

Shared menus can be opened by several viewers. Once opened, current viewers can
interact and all receive cell changes.

`Everyone` is useful for domain events such as global chat. Cell menus still
perform their own open/interact validation and viewer tracking.

`AudienceId` identifies sharing policy/state. It is not a world scope and not a
network address.

## Why audience and world scope remain separate

World scope answers:

> Which world namespace/source contains this state?

Audience answers:

> Who may observe or interact with this state?

Two chests in one world scope may have:

- public audience;
- personal audience;
- team audience;
- current-viewer audience.

The crafting table uses world scope as part of its menu and audience ID, but
that does not make the concepts identical.

## Server pipeline

```text
CellMenuServerSet::ReceiveRequest
  -> Validate
  -> Apply
  -> Sync
```

The network receiver emits transport-independent requests.

Feature mods validate kinds and anchors. The authoritative mod applies menu
state operations. The sync mod sends updates to viewers.

## Opening

The generic network request contains:

```text
kind: String
anchor: Option<BlockPos>
```

The receive mod emits:

```rust
CellMenuOpenIntent {
    player_id,
    kind,
    anchor,
}
```

A feature recognizes the kind and emits:

```rust
CellMenuOpenRequested {
    player_id,
    menu_id,
    title,
    audience,
    layout,
}
```

The authoritative store opens an existing compatible menu or creates it.

## Moving within a menu

The server:

1. verifies the viewer can interact;
2. calls generic inventory move/swap;
3. obtains changed cells;
4. emits one update for every current viewer.

Shared viewers see the same server state.

## Inventory bridge

`CellMenuMoveEndpoint` can refer to:

- player inventory cell;
- cell-menu cell.

The bridge server mod validates and applies cross-container swaps.

The client drag/drop bridge resolves source and target UI entities and emits one
local intention. An optimistic mod previews the swap in both caches.

Authoritative server updates correct the preview.

## Client UI layers

- `client-cell-menu-cache-mod`: active menu state;
- `client-cell-menu-ui-bevy-mod`: grid layout;
- `client-cell-menu-drag-drop-mod`: within-menu drag/drop;
- inventory bridge drag/drop mod;
- optimistic move mod;
- network send/receive mods;
- item quantity and favicon decorations reused through item instances.

Drag visuals must be placed above all target slots. Optimistic cache state
should hide the old item immediately, avoiding a one-frame return to its source
cell.

## Crafting table feature

The crafting table is a vanilla cell-menu behavior.

Its mechanic and policy are separate:

- `server-crafting-table-menu-lib` verifies the target and builds the menu
  request;
- `server-crafting-table-menu-vanilla-mod` applies that mechanic to every
  matching open intent.

A scoped custom server can omit the glue mod and call the library only for
selected players, nodes, permissions, or game phases.

Client:

1. a specific right-click handler sees `BlockId::CraftingTable`;
2. sends `CellMenuRequest::Open`;
3. marks the block-use operation handled.

Server:

1. receives `CellMenuOpenIntent`;
2. checks kind `demo:crafting-table`;
3. verifies the actor sees a crafting-table block at the anchor;
4. resolves resident key/world scope;
5. creates a shared menu ID from instance, provider, and position;
6. requests a `3x3` storage layout.

The cell-menu base contains no recipe logic.

## Adding a chest

A chest feature should provide:

- a block contributor;
- optional item contributor;
- client block-specific open handler;
- server validation of block and reach;
- stable scope/position-based `CellMenuId`;
- layout;
- audience policy;
- persistence policy.

Recipe code is not needed. A chest may use a larger storage layout.

For durable chests, persist menu contents independently from the generic
in-memory `ServerCellMenus`.

## Adding recipes

A crafting feature can listen after menu cell changes and:

1. inspect the `3x3` input;
2. match a generated recipe registry;
3. update an output section;
4. consume inputs only on output take;
5. emit authoritative cell updates.

This should be separate from:

- crafting-table open behavior;
- menu storage;
- drag/drop;
- network synchronization.

## Audience resolution and current limit

`server-audience-api` is the generic resolution seam for domains that need to
turn an `Audience` into player IDs. The selected basic implementation maps a
shared audience to all online players. Chat uses this service directly.

Cell menus additionally track active viewers and validate each interaction, so
they do not broadcast every menu mutation to the whole resolved audience.
The scope resolver interprets `Audience::Shared` as a scope node and returns
members in its subtree. Teams, distance, permissions, and world visibility may
still require a replacement resolver and matching domain validation. Audience
resolution must never be treated as sufficient authorization by itself.
