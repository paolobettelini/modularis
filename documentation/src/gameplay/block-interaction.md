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

Public server order:

```text
ServerBlockEditSet::Receive
  -> Collect
  -> Validate
  -> Apply
  -> Sync
```

### Receive

The network mod maps packet source address to an authenticated player and emits:

```rust
ServerBlockBreakRequested {
    player_id,
    position,
}
```

### Collect

The world edit mod creates:

```rust
PendingBlockBreak {
    player_id,
    position,
    allowed: true,
}
```

### Validate

The vanilla reach mod can deny pending operations. Other validators can enforce:

- permissions;
- protected regions;
- tool requirements;
- game mode;
- block hardness;
- damage on break;
- rate limits.

### Apply

Allowed edits call `ServerChunkWorld::break_block_for_player`.

The world route is resolved for the actor, so edits affect the correct scope.

The always-on world glue delegates to `server-block-edit-world-lib`. A custom
server can omit `server-block-edit-world-mod`, inspect the same pending request
inside its own scope or game rules, and call the library only where breaking is
valid.

Success emits:

```rust
ServerBlockBroken {
    player_id,
    scope,
    position,
    previous,
}
```

### Sync

The network sync mod sends changes only to players in the same world scope.

The client cache applies the authoritative block instance and requests remeshes.

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
