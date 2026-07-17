# Testing, debugging, and performance

Modular architecture needs tests at more than one level.

## Unit tests

Use direct crate tests for pure or local behavior:

- packed bit array read/write and repack;
- chunk palette growth and serialization;
- negative coordinate conversion;
- exact-edge voxel raycasting;
- collision landing and ledge movement;
- portal frame recognition;
- dimension registration;
- AO brightness;
- provider uniform-layer behavior.
- biome registry validation, climate determinism, and feature vertical bounds;

Pure API/support crates should have most of their behavior covered without
starting Bevy.

## ECS integration tests

For pipelines, create a small `App`, add only required messages/resources and
systems, then update it.

Useful cases:

- a validator denies before apply;
- a specialized inventory handler prevents generic swap;
- dimension change triggers reset before player reposition;
- network typed dispatch happens before feature receive;
- flight apply happens before sync;
- optimistic UI is replaced by authoritative state.

Public `SystemSet` order makes these tests easier.

## Composition checks

Always compose and check both active applications after changing shared domain
types:

```sh
patchwork compose \
  --modpack client \
  --modpacks-folder ./modpacks \
  --mods-folder ./mods \
  --cache ./build-client

cargo check --manifest-path build-client/client/Cargo.toml

patchwork compose \
  --modpack server \
  --modpacks-folder ./modpacks \
  --mods-folder ./mods \
  --cache ./build-server

cargo check --manifest-path build-server/server/Cargo.toml
```

Changes that require both:

- block/item/dimension contributors;
- metadata fields;
- network messages;
- shared serialized data;
- public API signatures;
- modpack imports.

## Test alternate compositions

The main client/server build cannot prove replaceability.

Useful composition fixtures:

- server base without vanilla rules;
- checkerboard provider instead of Perlin;
- primary provider without the vanilla biome pack;
- single-world routing without dimensions;
- client without graphics pack;
- client with outline feature;
- alternate player renderer;
- transport provider replacement.

Even a periodic compose/check test catches accidental concrete dependencies.

## Documentation build

From `documentation/`:

```sh
mdbook build
```

Keep `SUMMARY.md` links valid. The root `ARCHITECTURE.md` no longer exists; this
book is the canonical source.

## Debug generated output

When composition is wrong, inspect:

- generated `main.rs` for initialization order;
- generated Cargo manifest for selected providers;
- generated registry enum for missing contributor;
- generated packet enum and typed event;
- output assets for namespaced path;
- modpack import/ignore chain.

Do not patch generated output to continue. Fix the source metadata.

## Runtime logging boundaries

Log at the layer that understands the failure:

- framing errors in transport;
- unknown/unavailable route in world API caller;
- validation denial in gameplay validator;
- Blocky or voxel-model parse error in the selected model provider;
- missing asset in presentation;
- duplicate provider at registration.

Avoid logging the same failure in every layer.

## Chunk performance

Current safeguards:

- palette/bit-packed storage;
- zero-bit uniform sections;
- uniform provider fast paths;
- finite moving 3D window;
- request budget of four per frame;
- remesh budget of four per frame;
- retries instead of one-shot events;
- one-lock missing-position query;
- partial selection instead of full queue sort;
- deduplicated remeshes;
- air and fully hidden uniform mesh fast paths;
- material reuse.

When profiling vertical streaming, separate:

- server generation;
- network serialization;
- client cache insertion;
- mesh generation;
- Bevy asset/entity creation.

One combined frame time does not identify the correct mod to optimize.

## Network performance

TCP outboxes avoid blocking normal update systems on complete writes.

Current gaps:

- unbounded outboxes;
- no chunk response compression beyond compact CBOR data;
- no explicit request cancellation packet;
- no protocol metrics;
- synchronous packet encode/decode on update thread.

Improve these in transport/scheduling mods.

## Rendering performance

The active JSON model mesher emits the visible baked quads of every block. It
does not merge adjacent coplanar model faces.

Likely next improvements:

- greedy meshing;
- background mesh jobs;
- mesh pooling;
- texture arrays;
- frustum/distance culling;
- separate opaque and transparent passes.

Preserve `ChunkMeshApi` and `ChunkRenderApi` boundaries while iterating.

## Bevy query conflicts

If one system mutably queries the same component in two parameters, Bevy raises
`B0001`.

Make queries provably disjoint:

```rust
Query<&mut Transform, (
    With<BlockyModelNode>,
    Without<BlockyModelVisual>,
)>

Query<&mut Transform, (
    With<BlockyModelVisual>,
    Without<BlockyModelNode>,
)>
```

or use `ParamSet`.

This matters in modular systems where one feature introduces a new component on
entities already queried elsewhere.

## UI debugging

For drag/drop:

- inspect source and target entity markers;
- ensure one operation ID per logical drop;
- deduplicate Bevy pointer events in the same frame;
- update optimistic cache before rebuilding visuals;
- keep dragged visual above slots and decorations;
- verify authoritative packet returns both changed cells.

For settings:

- verify only one editor owns keyboard focus;
- separate stored type from editor provider;
- rebuild menus after providers register.

## Performance rule

Optimize behind the narrowest contract that owns the cost.

Do not merge modules only to gain access to private state. If an optimization
needs new information, add a small public summary or service to the relevant
API.
