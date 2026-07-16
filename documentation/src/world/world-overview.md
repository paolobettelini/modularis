# World architecture

The world subsystem separates data representation, source selection, cache
policy, client streaming, and rendering.

```text
Block and metadata registries
             │
       Chunk data format
             │
server provider registry
             │
viewer-aware chunk router
             │
resident cache + edit overlay
             │
chunk request validation
             │
network chunk response
             │
client cache
             │
streaming priority + remesh budget
             │
mesher + renderer
```

No single "world manager" owns this entire flow.

## Server concerns

The server decides:

- which provider answers a viewer's query;
- which world instance namespaces the data;
- which chunks may be requested;
- which chunks stay in the resident cache;
- how edits persist across cache eviction;
- which players receive changes.

## Client concerns

The client decides:

- which chunks are active around the camera;
- request order and retry policy;
- how responses are cached;
- which chunks need remeshing;
- how many remeshes happen per frame;
- how block faces and lighting become meshes;
- how mesh parts become Bevy entities.

## Coordinates are fully three-dimensional

Chunks are `16x16x16`, and `ChunkPos` has `x`, `y`, and `z`.

The world has no hard vertical limit. The client keeps a finite moving window
around the player, while providers can answer any integer chunk coordinate.
Uniform sky and deep layers are represented cheaply.

## Viewer-aware queries

The server world API takes a `ChunkViewer`:

```rust
pub enum ChunkViewer {
    Server,
    Player(PlayerId),
}
```

The same coordinates may resolve differently for different players. This
enables:

- dimensions;
- private instances;
- per-player worlds;
- provider routing based on permissions or game state.

The following chapters describe the data and flow in detail.
