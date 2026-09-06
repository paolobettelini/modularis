# Voxel frames and three-dimensional interest

A **voxel frame** is an independently positioned, rigid voxel grid inside a
world scope. It is not a dimension, a generator, a Patchwork mod, or a player
visibility group. A scope can contain its implicit root grid and any number of
runtime-created frames.

This refactor reuses the existing block-state palette, chunk codec, sparse
components and mesh builders. It does not introduce a second kind of block.

## Identity and ownership

The relevant keys answer different questions:

| Key | Meaning |
| --- | --- |
| World instance | Which independently stored/routed world owns this data? |
| Provider/source | Which terrain source within that world? |
| VoxelFrameId | Which local grid within this scope? |
| Local chunk/block position | Where in that grid? |
| Audience / scope facets | Who receives which gameplay information? |

`VoxelFrameId` is a runtime UUID, persisted with structural metadata. It must
not be generated from a mod list or from the frame's current pose.
`VoxelFrameId::ROOT` is the implicit identity grid in each scope. It cannot
be moved or registered as an extra frame and requires no catalogue entry.

`VoxelBlockAddress { frame, local }` and
`VoxelChunkAddress { frame, local }` prevent two frames with the same integer
coordinates from sharing cache entries, edits, damage or menus. Scope remains
on server resident/storage keys; the client holds its currently routed scope.
Root-only callers can still pass `BlockPos` or `ChunkPos` to the facade's
conversion-based convenience methods.

Chunks remain cubic 16 × 16 × 16, with the same negative-coordinate Euclidean
division and local index layout on every axis.

## Crate boundaries

| Concern | Owner |
| --- | --- |
| UUIDs, local addresses, rigid pose, bounds, physics seams | `voxel-frame-api` |
| Scoped structural registry and replaceable broad phase | `voxel-frame-registry-api` |
| World/local ray queries and oriented-box overlap | `voxel-frame-geometry-lib` |
| Structural binary catalogue codec | `voxel-frame-storage-lib` |
| Symmetric interest volumes and 3D distance | `chunk-interest-api` |
| Client frame state and render-parent handles | `client-voxel-frame-api` |
| Wire payloads / codegen contribution | `voxel-frame-network-message-types` / `voxel-frame-network-messages-mod` |
| Client wire adapter | `client-voxel-frame-network-mod` |
| Client pose-to-render transform adapter | `client-voxel-frame-render-mod` |
| Server viewer snapshots and pose replication | `server-voxel-frame-network-mod` |
| Registry changes to public ECS messages | `server-voxel-frame-events-mod` |
| Structural catalogue save/load adapter | `server-voxel-frame-persistence-mod` |
| Pending placement contract and phases | `server-block-placement-api` |
| Optional cross-frame placement exclusion | `server-voxel-overlap-lib` + `server-voxel-overlap-validation-vanilla-mod` |
| Reusable construction / privileged command | `server-voxel-frame-spawn-lib`, `server-command-spawn-frame-lib`, `server-command-spawn-frame-vanilla-mod` |
| Movement curves / client animation | `voxel-frame-movement-lib`, `client-voxel-frame-movement-api`, `client-voxel-frame-movement-mod` |
| Server presentation requests | `server-voxel-frame-motion-api` |

The data/math/codec crates are ordinary libraries, not exclusive Patchwork
providers. The network contributor is a support mod. Runtime adapters are
lifecycle mods. Existing world/provider, storage, chunk cache, mesher and
interaction APIs keep their separate responsibilities.

## Rigid coordinates

`VoxelFrameTransform` stores a double-precision translation and normalized
quaternion. Construction and deserialization reject non-finite values and
zero quaternions. There is no scale or shear.

Use its methods consistently:

- `local_to_world` and `world_to_local` for points;
- `local_direction_to_world` and `world_direction_to_local` for directions;
- `transform_bounds` for conservative world bounds;
- `render_transform(render_origin)` at the rendering boundary.

Do not add a frame's translation to voxel positions in storage. Do not bake its
rotation into mesh vertices. Moving a frame updates structural metadata and its
render parent, not the chunk palette, damage records or voxel mesh.

The client API exposes a double-precision render origin. This is an integration
seam, not a complete floating-origin implementation: an application that changes
it must also rebase cameras, players and other world-space consumers. Existing
player simulation and local DDA still use their established float precision.

## Creating and editing a frame

A custom server obtains the existing routed scope, registers a frame, and uses
the same authoritative world facade as for root blocks:

```rust,ignore
let viewer = ChunkViewer::Player(player_id);
let root = world.resident_key(viewer, ChunkPos::new(0, 0, 0))?;
let id = VoxelFrameId::new();
let pose = VoxelFrameTransform::new(
    [12.0, 8.0, -3.0],
    DQuat::from_rotation_y(0.4).to_array(),
)?;
world.frames().upsert(VoxelFrame::new(id, root.scope(), pose))?;

world.set_block_for(
    viewer,
    VoxelBlockAddress::new(id, BlockPos::new(-1, 0, 0)),
    selected_block,
)?;
```

This is an orchestration example; handle errors according to your application.
The block ID must come from that application's selected content, not a
hardcoded list in the frame core.

Extra frames have a sparse set of occupied local chunks. Unoccupied positions
read as air; they do not invoke the root terrain generator. First/last block
mutations update occupancy and conservative bounds. Empty frames have no
streamable geometry. Removing an entire transient instance also removes its
runtime frames.

Use `world.frames().set_transform(scope, id, pose)` to move a frame. Pose
changes leave voxel and component addresses unchanged. The registry emits
structural changes through the ECS bridge. Lifecycle code must use this shared
registry rather than mutating a detached copy of a frame descriptor.

## Spatial index and streaming

The registry keeps an index per scope. `VoxelFrameSpatialIndex` is replaceable;
the default median-split BVH filters bounds in all three axes. It rebuilds its
metadata tree when bounds change, not voxel meshes. For many frequently moving
frames, a dynamic tree or spatial hash can implement the same trait.

Root interest uses a shared `ChunkInterestVolume` contract. The default is:

```text
dx² + dy² + dz² <= radius²
```

The client default maximum radius is 8 chunks; server residency uses radius 9
for isotropic slack and a configurable maintenance interval. There is no
special short Y window or XZ-first priority.
`client-chunk-distance-priority-vanilla-mod` selects squared 3D distance
through the existing work-priority API.

Extra-frame interest first queries world bounds, then considers only occupied
local chunks whose transformed bounds intersect the world-space interest
sphere. It never enumerates an infinite grid for each frame. Bounds are
conservative, so a small number of extra candidates is expected.

Client request retries, deduplication and bounded remeshing remain active.
`ServerChunkRequestBudget` additionally bounds server generation work:
32 chunks per update, an 8 ms soft budget, 256 queued requests per player and
4096 in total by default. One chunk generation cannot be preempted; the time
budget is therefore soft. Custom orchestration can replace these resource
values. Requests are revalidated against player identity, epoch and interest
when dequeued. Dropped excess requests recover through client reconciliation.

## Rendering and selection

Chunk meshes, grass, damage overlays and outlines use frame-local geometry
under a shared `VoxelFrameEntity` parent. The independent render adapter
updates parent transforms before Bevy propagates global transforms.

Neighbor lookup carries the same frame ID: touching grids do not cull each
other's faces. Pose changes do not trigger voxel mesh regeneration.

Selection follows this chain:

1. Create a world-space ray and conservative segment bounds.
2. Query candidate frames in the active scope, including implicit root.
3. Transform the ray into each candidate's local coordinates.
4. Run the existing shape-aware voxel DDA.
5. Compare world distances and select the nearest hit.

A hit contains its frame-aware block and adjacent addresses, local face normal,
world normal and world hit position. Placement uses the **local** adjacent cell;
reach checks compare the player's world-space eye with the transformed target.
Outlines retain the model's precomputed exterior-edge geometry.

## Authoritative mutations and optional policy

Block breaking, survival damage and item placement carry frame addresses end to
end. The backend serializes mutations and performs occupied/air checks inside
the mutation boundary. Concurrent chunk misses cannot publish generated data
over a newer authoritative edit.

Placement exposes public phases:

```text
BlockPlacementSet::Collect -> Validate -> Apply
```

These are inside the inventory world-effects phase. Preparation is a reusable
function; the vanilla caller chooses when to invoke it. The optional overlap
mod rejects intersections with occupied shapes in other frames, using broad
phase bounds followed by oriented-box SAT. It also considers accepted pending
placements in the same update. Mere face contact is allowed.

Removing that policy mod allows overlapping frames; the chunk core does not
silently enforce a vanilla spatial rule. Custom code can call the overlap
library between preparation and commit, or use another policy altogether.

This is **placement validation**, not physical rigid-body simulation.
`VoxelFrameVelocity` and `VoxelFrameCollisionPolicy` expose a future
integration boundary. Kinematic player support/carrying is now implemented separately; see
[character collisions](../gameplay/character-collisions.md). A rigid-body frame
physics engine is still outside the project.

## Palette, properties and sparse data

The representation remains:

- `BlockState`: block type and low-cardinality discrete state in the palette;
- block properties: modular static/default values, not saved per position;
- block components: optional per-position records outside the palette.

A component key includes world scope, frame, local chunk and local block index.
Damage remains a sparse delta against default durability. An undamaged block
has no damage record, and restoring the default removes it. Equal local
coordinates in different frames have independent records.

Persistent components retain stable namespaced IDs, versions and opaque bytes.
Unknown components are preserved by the component codec. Persistent data is
not automatically part of normal chunk streaming: damage replication remains
its own feature adapter.

Crafting-table identity now includes scope, frame and local position. Different
frames cannot accidentally share a table menu. The menu remains a transient
shared 3×3 session managed by the cell-menu system; no persistent crafting
inventory or chest implementation is added. Vanilla opening applies world-space
reach validation; custom code may invoke the reusable opening mechanic under
its own conditions.

## Persistence domains

There are three independent payload families:

1. **Chunk data**: existing global block index, local subpalette and packed state.
2. **Block components**: sparse versioned records in their existing domain.
3. **Frame structure**: a versioned binary catalogue in
   `modularis:voxel-frames`.

Root region paths stay compatible with the existing world converter:
`data/chunk/regions/<source>/r.x.y.z.bin`.
Extra grids use
`data/chunk/regions/<source>/frames/<uuid>/r.x.y.z.bin`.
Here `<source>` means the filesystem adapter's encoded source key.

Generic world-data domains similarly add a frame segment only for non-root
keys. The structural catalogue stores IDs, scope, rigid pose, revision and
occupied local chunks; it does not contain chunk payloads or concrete gameplay
component structs.

Adapters queue dirty writes through existing storage services and their
periodic/shutdown flush. A frame pose change queues structure only.
Malformed stored chunks/catalogues produce errors instead of silently
regenerating and overwriting the saved world. This is buffered persistence,
not a transaction across every storage domain; crash-atomic multi-domain saves
would need a separate journal/transaction layer.

## Network lifecycle

The frame support contributor exports:

- `VoxelFrameUpsert`: complete structural snapshot;
- `VoxelFramePose`: pose, revision and server clock sample;
- `VoxelFrameRemove`: stop displaying a frame.

All carry a movement epoch and stream sequence. The server snapshots after
relocation synchronization; the client merges typed packet readers by sequence
before applying operations. This matters when leave/re-enter packets arrive
in the same update. Epoch checks discard packets from before a relocation.

Chunk requests/responses also carry the movement epoch, and chunk responses
identify their frame independently from the unchanged local chunk payload.
World/dimension reset clears active frame state and child entities. Recompose
both client and server: the wire schema changed.

## Spawning a frame on demand

The default server no longer constructs the tilted sample at startup.
Use the privileged command:

```text
/spawnframe 10 8 0 15 35 -10
```

The first three numbers are world position. The last three are Euler rotations
about X, Y, Z **in degrees**, composed using quaternion Euler XYZ convention.
They are angles, not a direction vector or angular velocity. The origin is the
corner of local block (0,0,0), not its center.

The command creates a UUID frame in the caller's routed world, with one stone
block at local (0,0,0). Its command library takes the initial block as input;
only the vanilla glue selects Stone and explicitly depends on `block-stone`.
A custom server can invoke `server_voxel_frame_spawn_lib::spawn_frame` without
installing the command or choosing stone.

The former demo mod remains available but is not selected. Previously saved
demo frames are not destructively deleted when changing a modpack.

## Animated pose packets

`VoxelFramePose` now includes `FrameMovement`:

- `Instant`: apply the target directly;
- `Animated`: duration in milliseconds, independent translation and rotation
  easing, and `Once`, `Loop` or `PingPong` repetition.

`Easing` supports Linear and CSS cubic Bézier curves. The implementation solves
X(t) for the elapsed fraction before evaluating Y(t), rather than treating
elapsed time as the Bézier parameter. X control points must be in [0,1]; all
control points must be finite. Y can overshoot. Zero duration is instant,
including for repeat modes.

The network adapter validates ordering and starts/replaces a track. The
independent client movement mod advances it using translation lerp and quaternion
slerp. New motion starts at the current displayed pose, not the previous target.
Snapshots, instant poses, frame removal and world/disconnect reset cancel tracks.
Animation updates frame bounds for client queries without rebuilding voxel meshes.

A server mod can write this ECS message, preferably before
`VoxelFrameSet::Replicate`:

```rust,ignore
motions.write(SendVoxelFrameMotion {
    player_id,
    frame: frame_id,
    target: target_pose,
    movement: FrameMovement::Animated {
        duration_ms: 1200,
        translation_easing: Easing::CubicBezier {
            x1: 0.42, y1: 0.0, x2: 0.58, y2: 1.0,
        },
        rotation_easing: Easing::Linear,
        repeat: RepeatMode::PingPong,
    },
});
```

The server network adapter supplies the recipient's epoch, ordered sequence
and frame revision. Requests for frames not visible to that recipient are
ignored. Ordinary authoritative registry pose changes still send Instant.

For actual moving collision geometry use `SetVoxelFrameMotion` from
`voxel-frame-kinematic-api` and the server kinematic adapter. See
[character collisions](../gameplay/character-collisions.md).

`voxel-frame-kinematic-api` is a Patchwork API and
`server-voxel-frame-kinematic-mod` is its default provider. Gameplay depends
on the API contract; a composition selects the provider. A custom server can
therefore replace trajectory simulation without changing the gameplay mod.

`SetVoxelFrameMotion` also carries `initial_elapsed_seconds`. Leave it at
`0.0` for an animation that starts at its first endpoint. A deterministic
procedural feature can provide an offset within a repeating trajectory so that
different frames do not all start in phase. The kinematic adapter stores the
offset by backdating `FrameTrajectory::started_at_seconds`; the existing
trajectory packet then gives late viewers the same authoritative phase.

The `SendVoxelFrameMotion` message requests **client presentation**, not server simulation:
it does not move collision geometry, change persistence or extend authoritative
streaming interest along an animation path. A future moving-frame authority
must drive those separately. The client starts at receipt time; the server clock
sample is retained for a future synchronized-clock adapter, not incorrectly
compared with a client's unrelated startup clock. Loops jump back to the start;
ping-pong reverses direction continuously. The TheCrown parkour is the first
gameplay consumer of authoritative animated frame motion.

Root-axis portal mechanics remain root-only. They do not attempt to interpret
a tilted frame as a vertical portal. A rotated-portal feature can be added
separately.

## Verification and extension checklist

Focused invariant checks cover rigid transforms, negative local coordinates,
isotropic interest, nearest rotated hits, SAT contact, equal local positions in
different frames, structural save/load, sparse damage and menu identity.
Compile representative client, vanilla server and TheCrown compositions after
changing public addresses or packet types.

For a new feature ask:

- Is this data structural, static property, sparse component or UI session?
- Does every key preserve frame and scope?
- Does a pose change leave saved local data untouched?
- Are world/local conversions centralized?
- Can its policy be omitted without breaking the underlying operation?
- Is replication explicitly selected rather than implied by persistence?
