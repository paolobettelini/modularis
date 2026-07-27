# Inventory, hotbar, and item use

Inventory is a server-authoritative generic layout. It does not hardcode stacks
or a fixed hotbar size.

## Core data

```rust
pub struct InventoryLayout {
    pub sections: Vec<InventorySectionLayout>,
}

pub struct InventorySectionLayout {
    pub id: InventorySectionId,
    pub role: InventorySectionRole,
    pub columns: u32,
    pub cells: u32,
}

pub struct InventoryCell {
    pub section: InventorySectionId,
    pub index: u32,
}

pub struct Inventory {
    pub layout: InventoryLayout,
    cells: HashMap<InventoryCell, ItemInstance>,
}
```

Section roles are currently:

- `Storage`;
- `Hotbar`.

The hotbar is a section of inventory, not a separate container type.

## Core semantics

The base inventory supports:

- layout validation;
- cell lookup;
- set/remove;
- move or swap;
- resize.

It does not implement:

- stack merging;
- quantity;
- consumption;
- item use;
- UI decoration.

`move_or_swap` only moves one `ItemInstance` or swaps two instances.

## Server pipeline

```text
InventoryServerSet::ReceiveRequest
  -> Validate
  -> DispatchUse
  -> ApplyWorldEffects
  -> ApplyConsumption
  -> Sync
```

Validation has subgroups:

```text
Initialize -> Stack -> MoveOrSwap -> Other
```

This lets stacking claim an operation before generic swapping.

## Client intentions

The client emits:

- move cell;
- select hotbar;
- use held item;
- request synchronization.

Network receive maps the authenticated address to a player and emits
authoritative requests.

The server never accepts a complete inventory snapshot from the client.

## Authoritative state

`ServerInventories` maps owner ID to:

```rust
ServerPlayerInventory {
    inventory,
    selected_hotbar,
}
```

The authoritative mod:

- applies reset/resize/set requests from trusted server features;
- applies valid hotbar selection;
- handles unclaimed move/swap;
- dispatches held-item use from the selected cell;
- responds to sync requests;
- removes state when a player leaves.

## Reset, resize, and cell updates

Server features request changes through ECS messages:

- `InventoryResetRequested`;
- `InventoryResizeRequested`;
- `InventorySetCellRequested`.

Applied messages are then synchronized:

- `InventoryResetApplied`;
- `InventoryResized`;
- `InventoryCellSet`;
- `HotbarSelectionSet`.

Network packets mirror these results.

The server chooses layout dimensions. The client renders the received layout.

## Default layout and loadout

`server-inventory-layout-default-impl` provides the demo layout.

`server-inventory-default-loadout-mod` listens to joins and missing-inventory
sync requests. It creates block items and flint-and-steel with generated
metadata.

Most default items have infinite quantity for testing. Grass has a finite stack.

The loadout mod is vanilla policy. A custom server can replace it without
changing inventory storage or synchronization.

The item construction itself is exposed by
`server-inventory-default-loadout-lib`, so an orchestrator may grant that
loadout only in selected scopes or use it as the starting point for another
policy.

## Stack handling

`server-inventory-quantity-stacking-mod` runs before generic move/swap.

It merges only when:

1. source and target item IDs match;
2. both have quantity metadata;
3. all metadata except quantity is equal.

Finite quantities use saturating addition. If either quantity is infinite, the
result is infinite.

Stack compatibility and quantity merge live in
`inventory-quantity-operations-lib`; the selected stacking mod only connects
those operations to the authoritative inventory pipeline.

After handling it emits `InventoryMoveHandled`, preventing generic swap.

This demonstrates why quantity is metadata rather than a field on every slot.

## Item use

The server authoritative mod resolves the selected hotbar cell and emits:

```rust
HeldItemUseDispatched {
    player_id,
    cell,
    item,
    target,
}
```

Feature mods listen during `ApplyWorldEffects`.

Current consumers:

- block placement from `PlaceBlock`;
- portal ignition from `PortalIgniter`.

On a successful effect they emit:

```rust
ItemUseSucceeded {
    player_id,
    cell,
    item_before_use,
}
```

## Quantity consumption

The separate consumption mod runs in `ApplyConsumption`.

It:

- verifies the cell still contains the same item;
- ignores missing quantity metadata;
- ignores infinite quantity;
- removes `Finite(1)` or `Finite(0)`;
- decrements larger finite quantities;
- emits an authoritative cell update.

World effects do not decrement items themselves.

## Client cache and optimistic movement

Client layers:

- authoritative inventory cache;
- optimistic move preview;
- network send/receive;
- inventory UI;
- drag/drop behavior;
- hotbar UI;
- quantity decoration;
- favicon decoration.

The client can immediately preview a swap. The next server update remains
authoritative and corrects rejected or transformed operations.

Operation IDs let server-side specialized handlers and client-side pending state
refer to one move.

## Item rendering

The base slot UI emits `InventorySlotVisualCreated`.

Decoration mods add:

- item favicon;
- finite quantity number;
- infinity glyph using the DejaVu font resource.

When favicon metadata exists, the fallback item name is hidden.

The quantity visual must have a higher UI z-index than the favicon.

## Extending inventory

Examples:

- add durability metadata and a durability bar mod;
- add equipment sections and validation;
- add maximum stack sizes;
- add item binding/ownership;
- cause damage when a move succeeds;
- reject moves while stunned;
- add creative inventory behavior.

Put semantics in a feature mod that reads requests or applied events. Do not add
every rule to `Inventory::move_or_swap`.

## Adding a server-defined layout

Implement `ServerInventoryLayoutApi` and select it as the provider.

Keep layout construction separate from loadout. Two servers may share the same
layout and assign different items.
