# Client streaming, meshing, and rendering

The client chunk pipeline is split so that range policy, network recovery,
cache state, meshing, lighting, and Bevy rendering can change independently.

## Pipeline

```text
camera position
  -> ActiveChunks
  -> missing-chunk reconciliation
  -> priority-ranked requests
  -> ChunkResponse
  -> ClientChunkCache
  -> deduplicated remesh queue
  -> ChunkMeshService
  -> Bevy mesh entities
```

## Streaming window

`client-chunk-streaming-around-player-impl` reads:

- camera transform;
- render distance setting;
- `ChunkStreamingViewConfig`.

The default view config allows:

- horizontal radius up to 8;
- vertical radius 2.

The center uses all three camera coordinates. The window follows arbitrary
positive and negative Y chunk positions.

The implementation rebuilds the desired `HashSet<ChunkPos>` only when:

- center changes;
- horizontal radius changes;
- vertical radius changes;
- active state was explicitly cleared.

It emits:

- `ChunkNeeded` for new positions;
- `ChunkUnload` for removed positions.

## Active state and recovery

`ActiveChunks` plus `ClientChunkCache` are recoverable source of truth.

The request mod does not rely only on one `ChunkNeeded` event. Every frame it:

1. removes pending requests for inactive chunks;
2. finds active positions missing from cache under one cache read lock;
3. ensures each missing position has a pending request;
4. retries sent requests after 0.5 seconds.

This recovers from:

- event ordering races;
- rejected or lost application-level requests;
- dimension cache resets;
- temporary connection timing.

TCP is reliable at the byte-stream level, but application state may still miss
a response due to lifecycle or validation, so reconciliation remains useful.

## Work priority

Request and remesh order share `ChunkWorkPriorityService`:

```rust
priority(position, focus) -> ChunkWorkPriority
```

The neutral provider preserves FIFO-compatible ordering.

The vanilla layered policy ranks:

1. absolute vertical layer distance from focus;
2. horizontal distance.

Therefore the current XZ plane fills first, nearest chunks first, before upper
and lower layers. This reduces visible stutter while crossing a vertical chunk
boundary.

A custom client can provide:

- Euclidean distance;
- camera view-cone priority;
- forward-motion prediction;
- teleport destination priority;
- separate request and remesh policies.

## Request budget

The client sends at most four chunk requests per frame.

It uses partial selection to find the best four instead of sorting every
pending request. The selected subset is then sorted for stable best-first
processing.

This bounds network bursts and per-frame CPU work.

## Client cache

`ClientChunkCache` is a lock-protected `ChunkPos -> Chunk` map.

It supports:

- insert/remove/clear;
- chunk and block queries;
- block mutation from authoritative edit packets;
- uniform-block inspection;
- bulk missing-position checks.

Chunk responses emit `ClientChunkAvailable`.
Block updates emit `ClientChunkChanged`.

Dimension changes clear the cache and active window only when the dimension
actually changes, not for a repeated initial notification.

## Mesh neighborhood

The mesher receives a `ChunkMeshNeighborhood`:

- center chunk;
- available neighbor chunks;
- world-space block lookup across chunk boundaries.

Missing neighbors are treated as transparent. When a neighbor arrives or
changes, the renderer remeshes affected chunks.

The renderer includes all 26 neighboring chunk positions for invalidation
because vertex ambient occlusion can sample across edges and corners, not only
six face neighbors.

## Naive cube mesher

`client-chunk-mesh-naive-cubes-impl`:

1. iterates all blocks in the center chunk;
2. ignores air and invisible shapes;
3. tests each of six neighboring blocks;
4. emits only faces adjacent to a non-opaque or missing block;
5. groups mesh parts by texture;
6. computes optional vertex lighting;
7. chooses the better quad diagonal for AO gradients.

Each mesh part contains positions, normals, colors, UVs, and indices.

The API is replaceable. A greedy mesher should implement the same
`ChunkMeshApi` and install a `ChunkMeshService`.

## Empty and uniform fast paths

The mesher returns no geometry when:

- center chunk is uniform air;
- center chunk is uniform opaque and all six face-neighbor chunks are uniform
  opaque.

The renderer also skips uniform air chunks before meshing.

Receiving or unloading a uniform air chunk avoids unnecessary neighbor remeshes
because absence is already treated as transparent.

These paths are especially useful for vertically unbounded empty sky and deep
homogeneous terrain.

## Remesh queue and budget

The renderer stores a deduplicated `HashSet<ChunkPos>` of pending remeshes.

`ChunkRemeshBudget` defaults to four chunks per frame. Work is selected with the
same priority service used by network requests.

For each selected chunk:

1. remove old render entities;
2. read the cached center and neighbors;
3. run the selected mesher;
4. create or reuse texture materials;
5. create Bevy mesh entities at the chunk world origin;
6. update `RenderedChunks`.

Unloading removes both pending work and rendered entities.

## Material behavior

Mesh parts with a texture share a cached `StandardMaterial` by texture path.
Untextured visible blocks share a white material.

Current materials:

- use roughness 1.0;
- disable back-face culling;
- receive vertex colors.

Back-face culling is disabled to avoid visible holes while mesh orientation and
special shapes are still simple. A stronger renderer can enable it after
guaranteeing winding consistency.

## Extending client chunks

Common replacements:

- streaming policy: implement another `ChunkStreamingApi`;
- priority policy: provide `ClientChunkWorkPriorityApi`;
- cache: provide `ClientChunkCacheApi`;
- mesher: provide `ChunkMeshApi`;
- renderer: provide `ChunkRenderApi`;
- vertex lighting: register stages or replace the pipeline.

Avoid combining these into one "optimized chunks mod". Their independent
budgets and data contracts are what let the client evolve safely.
