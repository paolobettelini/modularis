# Block interaction and world edits

Block interaction separates raycasting, block-specific use, held-item fallback,
server validation, mutation, and synchronization.

## Voxel raycast

`voxel-raycast-api` implements a 3D DDA traversal. Its shape-aware entry point
uses DDA only to choose candidate cells, then intersects the local AABB union
provided by `BlockShapeService`.

It returns:

```rust
VoxelRayHit {
    block,
    adjacent,
    normal,
    distance,
}
```

`block` is the selected voxel. `adjacent` is the voxel outside the exact model
element face that was hit. Rays pass through empty regions of partial blocks;
for example, the empty upper half beside a lower slab is not selectable.

The traversal advances several axes only on an exact equal-time grid crossing.
It does not use a broad epsilon. Treating a near-edge ray as an exact edge can
select a diagonal voxel and place a block on the wrong face.

## Client reach policy

`client-block-interaction-rules-api` exposes `max_reach`.

`client-block-interaction-rules-vanilla-mod` provides the demo value.

The raycast feature depends on the API, not on a hardcoded constant. A custom
client can replace it, but server validation remains required.

## Client input flow

When playing:

1. the input backend records left/right mouse press;
2. raycast searches the client chunk cache;
3. left click emits `BlockBreakRequested`;
4. right click emits `LocalBlockUseIntent`;
5. the resulting target carries the exact face normal and adjacent cell.

The raycast mod does not decide whether a placement overlaps a player. The
server placement policy performs that authoritative check against the placed
block's actual shape. This avoids coupling a generic input/raycast feature to
one client player renderer or visibility policy.

## Right-click handler chain

```text
ClientBlockInteractionSet::Raycast
  -> RoutingRules
  -> SpecificHandlers
  -> Fallback
```

`RoutingRules` lets optional behavior redirect an operation before a block
handler claims it. The vanilla sneak routing mod marks the operation handled
and emits held-item use directly. Consequently, right-clicking a crafting table
while sneaking places or uses the held item instead of opening the table.

The crafting-table handler recognizes a crafting table and sends a menu open
request. It ignores operations already handled by routing rules, then emits
`LocalBlockUseHandled` for operations it claims.

The held-item fallback forwards only operation IDs not claimed by a specific
handler.

This allows more mods to add:

- doors;
- chests;
- machines;
- buttons;
- custom block GUIs.

They do not need to edit one central right-click match.

## Block break pipeline

Holding the left mouse button emits break intentions while the same block is
targeted. Packets are intentions, not direct mutations.

Public server order is split across two contracts:

```text
ServerBlockEditSet
  Receive -> Collect -> Validate -> Apply -> Sync

ServerBlockBreakingSet
  Dispatch -> ApplyEffects
```

### Receive and collect

The network mod maps packet source to an authenticated player and emits
`ServerBlockBreakRequested`. `server-block-breaking-events-mod` checks the
generic `CanInteract` capability and creates a `PendingBlockBreak`.

### Validate

The vanilla reach mod can deny pending operations. Other validators can enforce
protected regions, tools, game phase, scope-specific rules, or rate limits. A
validator does not need to know whether the final policy will break instantly
or accumulate damage.

### Dispatch

An allowed request is resolved against the actor's current world route. The
generic dispatcher publishes:

```rust
ServerValidatedBlockBreak {
    player_id,
    mode,
    key,
    position,
    block,
}
```

This event is the extension point for mode- or game-specific mechanics.

### Apply effects

The vanilla composition selects two independent listeners:

- Creative calls `server-creative-block-breaking-vanilla-lib` and mutates the
  block immediately;
- Survival calls `server-survival-block-breaking-vanilla-lib`, accumulates the
  sparse `BlockDamage` component, and mutates only when durability is exhausted;
- Adventure has no break listener.

Repeated packets from one player in one server update count once. Distinct
players targeting the same resolved world position contribute together, so
cooperative breaking is faster. Stopping does not reset damage.

The world route is resolved before grouping, so identical coordinates in two
instances do not share damage. Successful mutation emits `ServerBlockBroken`.
Replacing or breaking a block clears its per-position components.

The older `server-block-edit-world-mod` remains an optional instant-break glue
for custom compositions, but it is not selected by the vanilla server together
with the mode-aware pipeline.

### Persistence and synchronization

Damage is saved by the independent block-component persistence domain, not in
the chunk palette. `ServerBlockDamageChanged` is replicated only to viewers in
the same world scope. Existing damage is sent after a chunk is streamed, while
normal chunk packets remain unaware of block components.

The runtime ordering is explicit: validated break effects run in
`ServerBlockBreakingSet::ApplyEffects`, damage replication runs afterward in
`ServerBlockEditSet::Sync`, the client receives packets after protocol dispatch,
and client damage state is applied before drawing. This keeps the first visible
crack and the persistent sparse value in the same authoritative update path.
Only discrete stage transitions are replicated; holding on the same stage does
not resend an identical packet every server tick, and unbreakable blocks do not
send meaningless stage-zero updates.

The current path logs each boundary needed to diagnose a missing break:
transport receipt, permission/reach validation, scoped world lookup, durability
stage, audience replication, and final world mutation. The client logs received
damage stages and warns when an overlay cannot find its chunk or block shape.
These logs belong to the adapters that own each boundary; the durability
library remains free of transport and rendering concerns.

The selected input backend publishes the frame state in
`ClientInputSet::Capture`. Raycasting is explicitly ordered after this set, and
network sending is ordered after raycasting, so a feature never depends on an
accidental Bevy system order to observe a held mouse button.

The client cache applies authoritative block states and requests remeshes. A
separate overlay mod renders the six discrete damage stages against the block's
actual shape. See
[Block properties and sparse components](../world/block-properties-and-components.md).

## Placement through item use

Placement is not a direct right-click packet.

```text
right-click target
  -> LocalUseHeldItemIntent
  -> UseHeldItemRequest
  -> UseHeldItemRequested
  -> HeldItemUseDispatched
  -> server-place-block-item-use-mod
```

The vanilla placement mod requires:

- `PlaceBlock` metadata;
- a block target with adjacent position;
- authoritative reach;
- resolvable world scope;
- no visible player's current per-player hitbox overlapping any AABB of the
  placed shape;
- target block is air.

The full check and apply operation lives in
`server-place-block-item-use-lib`; the mod is only the blanket ECS listener.
This lets a custom game reuse `PlaceBlock` semantics conditionally without
enabling placement in every runtime node.

It calls `place_block_for_player` and emits:

- `ServerBlockPlaced`;
- `ItemUseSucceeded`.

Quantity consumption listens to success separately.

## Adding a block edit validator

```rust
fn protect_spawn(
    mut pending: ResMut<PendingBlockBreaks>,
) {
    for request in &mut pending.breaks {
        if request.position.x.abs() < 16
            && request.position.z.abs() < 16
        {
            request.allowed = false;
        }
    }
}
```

Register it in:

```rust
ServerBlockEditSet::Validate
```

If several validators need reasons, extend the pending decision contract rather
than relying on system order to overwrite a boolean.

## Block outlines

Outlines are an independent client feature family:

- API defines owner-keyed outline commands;
- Bevy provider renders thin edge meshes for every box in the selected shape;
- looked-block vanilla policy performs the same shape-aware raycast used by
  interaction.

An owner key lets several mods maintain independent outlines.

The active client selects the renderer in `client.toml` and the optional
looked-block policy in `client-vanilla.toml`. Omitting the policy leaves the
generic outline provider available for other features; omitting the provider
leaves outline commands without a renderer.

Crosshair and outline do not own, spawn, or reposition each other.

The server may independently enable or disable the whole outline capability per
player through `SetPlayerOutline`. Its network adapter sends
`SetOutline(bool)`; the client keeps owner-keyed outline state but hides all
roots while disabled. Adventure mode uses this policy, but the packet and state
APIs do not depend on game modes.

The selected vanilla break, placement, crafting-table, and portal-ignite glue
also requires effective `CanInteract` on the server. The pure mechanic
libraries do not: custom orchestration may substitute a scoped or
application-specific authorization rule before calling them.
