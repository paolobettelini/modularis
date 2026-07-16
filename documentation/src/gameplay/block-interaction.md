# Block interaction and world edits

Block interaction separates raycasting, block-specific use, held-item fallback,
server validation, mutation, and synchronization.

## Voxel raycast

`voxel-raycast-api` implements a 3D DDA traversal.

It returns:

```rust
VoxelRayHit {
    block,
    adjacent,
    normal,
    distance,
}
```

`block` is the solid voxel hit. `adjacent` is the empty voxel immediately
outside the entered face.

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
5. placement is rejected locally if the target voxel overlaps local or visible
   remote player hitboxes.

The local hitbox check is immediate feedback. The server performs its own
authoritative check.

## Right-click handler chain

```text
ClientBlockInteractionSet::Raycast
  -> SpecificHandlers
  -> Fallback
```

The crafting-table handler recognizes a crafting table and sends a menu open
request. It emits `LocalBlockUseHandled`.

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
- no player hitbox in the target voxel;
- target block is air.

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
- Bevy/Gizmos provider draws them;
- looked-block vanilla policy performs its own raycast.

An owner key lets several mods maintain independent outlines.

The active vanilla client currently selects the crosshair but does not include
the outline renderer/policy in `client-vanilla.toml`. Add both optional mods to
enable target highlighting.

Crosshair and outline do not own or reposition each other.
