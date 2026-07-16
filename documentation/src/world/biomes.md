# Biomes and world-generation features

The Overworld generator is assembled from four independent layers:

```text
generated BiomeId
        │
runtime biome definitions + feature registry
        │
viewer/instance-aware biome selector
        │
biome-driven primary chunk provider
```

This separation is important. A biome ID is shared domain identity. Climate
selection is policy. A cave or tree is an optional generation feature. Turning
those pieces into a `Chunk` is one provider implementation. A custom server can
replace one layer without forking the other three.

## Generated biome identity

Every biome contributor is a small mod with one namespaced ID:

```toml
[package.metadata.biome]
id = "example:lavender-fields"
```

`biome-registry-codegen` scans the final composed project and generates:

```rust
pub enum BiomeId {
    Desert,
    Forest,
    Plains,
    RockyPeaks,
    Tundra,
}

pub const ALL_BIOMES: &[BiomeId];

pub fn from_str(id: &str) -> Option<BiomeId>;
pub fn id(biome: BiomeId) -> &'static str;
```

Only `id` belongs in Cargo metadata. Terrain blocks, climate targets, visuals,
and features stay in Rust code. This keeps the manifest a contributor discovery
surface instead of turning it into a second programming language.

Duplicate namespaced IDs and duplicate generated variant names fail during
composition. The selected contributors live in `biomes.toml`; removing one and
recomposing removes its enum variant.

The identity contributor and server definition are separate mods. For example:

```text
biome-forest
    declares demo:forest for codegen

server-biome-forest-vanilla-mod
    registers the demo server's runtime Forest definition
```

This allows a client or shared protocol to know a biome ID without importing a
server terrain policy. The current protocol does not transmit biomes, so the
biome pack is selected only by the vanilla server pack for now.

## Runtime definitions

`server-biome-api` defines the data registered for each selected biome:

```rust
pub struct BiomeDefinition {
    pub id: BiomeId,
    pub name: &'static str,
    pub climate: BiomeClimate,
    pub terrain: BiomeTerrain,
    pub visuals: BiomeVisuals,
    pub features: Vec<BiomeFeatureId>,
}
```

### Climate

`BiomeClimate` contains:

- target temperature;
- target humidity;
- target continentalness;
- precipitation capability;
- downfall amount.

The first three values are targets, not hardcoded ranges. A selector decides
how to interpret them. The vanilla selector samples broad noise fields and
chooses the registered definition with the nearest climate point. A different
selector may use ranges, weighted tables, latitude, player state, a saved map,
or no climate values at all.

### Terrain

`BiomeTerrain` contains:

- base height;
- broad height variation;
- detail variation;
- surface block;
- subsurface block;
- underground block;
- subsurface depth.

These fields describe the current heightmap provider. They are not universal
requirements for every possible chunk provider. A floating-island or
file-backed provider may ignore them while still using `BiomeId` and registered
features.

### Visuals

`BiomeVisuals` currently records:

- sky color;
- fog color;
- water color;
- grass tint;
- foliage tint.

They are code-side biome data and an extension point for future client
synchronization. The current chunk protocol does not send a biome map, and the
renderer does not apply these tints yet.

### Feature list

A definition stores an ordered list of `BiomeFeatureId`. It does not call
feature implementations directly:

```rust
features: vec![
    caves_feature_id(),
    ores_feature_id(),
    dense_oak_trees_feature_id(),
]
```

The concrete feature mods register those IDs in `ServerBiomeRegistry`. Missing
features are reported when the selected biome provider first validates the
composed registry. Duplicate biome or feature registrations are rejected.

## World-generation feature contract

A feature implements:

```rust
pub trait ServerBiomeFeature: Send + Sync + 'static {
    fn phase(&self) -> BiomeFeaturePhase;

    fn vertical_range(&self) -> FeatureVerticalRange {
        FeatureVerticalRange::Any
    }

    fn generate(&self, context: &mut BiomeFeatureContext<'_>);
}
```

The phases are:

```text
Carving
  -> Underground
  -> Surface
  -> Decoration
  -> Finalization
```

Examples:

- caves run in `Carving`;
- ore replacement runs in `Underground`;
- ice patches run in `Surface`;
- trees, cacti, and boulders run in `Decoration`.

The definition's feature order is preserved within the same biome and phase.
The explicit phases let independent mods cooperate without private
`.before(function)` relationships.

### Vertical bounds

Each feature declares a conservative vertical range:

```rust
FeatureVerticalRange::Absolute { min: -48, max: 32 }

FeatureVerticalRange::RelativeToSurface { min: 1, max: 8 }
```

The provider uses this information to avoid running surface decorations in a
deep chunk or scanning cave logic in very high sky chunks. A feature must never
declare a range narrower than the blocks it can write. An incorrect bound can
make valid feature blocks disappear.

### Feature context

`BiomeFeatureContext` exposes:

- the complete `ChunkGenerationRequest`;
- current chunk position;
- target biome and its definition;
- deterministic world seed;
- biome and surface-height sampling at arbitrary X/Z coordinates;
- current-chunk block reads;
- clipped world-position writes;
- deterministic position hashing.

Writes are intentionally clipped to the chunk being generated. A feature that
crosses chunk boundaries evaluates neighboring anchors, then writes only the
part inside the current chunk. The same feature is evaluated independently when
the neighbor chunk is generated.

The oak-tree feature demonstrates this pattern:

1. scan candidate anchors two blocks beyond the current X/Z edge;
2. choose anchors with a deterministic hash;
3. derive trunk height from the same hash;
4. attempt all trunk and crown writes in world coordinates;
5. let the context keep only writes inside the current chunk.

This produces the same tree regardless of chunk request order and avoids a
mutable cross-chunk generation queue.

## Biome selection

Biome selection is an exclusive provider API:

```rust
pub trait ServerBiomeSelector {
    fn select(
        &self,
        request: &BiomeSelectionRequest<'_>,
        definitions: &[BiomeDefinition],
    ) -> Option<BiomeId>;
}
```

`BiomeSelectionRequest` includes:

- the original `ChunkGenerationRequest`;
- world X and Z.

The original request contains viewer, world instance, and chunk position. A
custom selector can therefore choose biomes per instance or even per player.
If selection varies by player, routing and edit scope must use matching world
identity; otherwise two players could edit logically different terrain under
the same resident key.

### Vanilla climate selector

`server-biome-climate-selector-vanilla-mod`:

1. derives a stable seed from `WorldInstanceId`;
2. samples low-frequency temperature, humidity, and continentalness noise;
3. compares the sample with every registered biome climate target;
4. selects the closest target with stable ID tie-breaking.

The area near the shared spawn is forced to Plains when that definition is
present. This preserves the server's spawn-at-Y=2 contract and avoids spawning
inside a tree, cactus, or steep peak.

This selector is vanilla policy. Another provider can select from a saved biome
map, use Voronoi cells, attach biome state to world instances, or expose a
scripted query service.

## Biome-driven chunk provider

`server-chunk-provider-biomes-mod` is the selected primary Overworld provider.
It implements the normal `ServerChunkProvider` contract and registers
`ChunkProviderId::primary()`.

For each request it performs the following work.

### 1. Validate the selected registry

Every feature referenced by every definition must have a registered
implementation. Validation happens lazily once, after all Patchwork mods have
completed initialization.

### 2. Create a request-local sampler

The sampler owns request-local caches for:

- `(x, z) -> BiomeId`;
- `(x, z) -> surface height`.

Generation features repeatedly ask the same biome and height questions. These
caches avoid repeating climate and terrain noise for every block.

### 3. Build column samples

For each of the 256 X/Z columns, the provider records:

- selected biome;
- copied terrain parameters;
- surface height.

Height parameters are blended across nearby biome samples before noise is
applied. Surface block identity remains biome-specific, while height transitions
are less likely to create a vertical wall at a climate boundary.

The shared spawn blends from height one into distant generated terrain.

### 4. Find active biomes and features

The provider samples a small margin around the chunk. This includes tree crowns
and boulders whose anchor lies in a neighboring chunk.

It then resolves the feature lists for every biome present in that area and
orders them by public phase.

### 5. Use uniform fast paths

Before visiting 4096 cells, the provider compares the vertical chunk range with:

- minimum and maximum local surface height;
- active feature bounds;
- maximum subsurface depth;
- common underground block.

It can return immediately when the result is guaranteed to be:

- uniform air above terrain and all relevant decorations;
- uniform underground material below all relevant feature ranges.

`Chunk::filled` creates a one-entry palette with zero packed data words. Empty
sky therefore stays cheap in memory, on the network, and in the client mesher.

### 6. Generate base strata

Mixed chunks receive:

```text
air
surface block
subsurface block for configured depth
underground block below it
```

There is no hard vertical world boundary. Very deep chunks become homogeneous
stone after absolute cave/ore feature ranges end.

### 7. Apply features

The provider runs active features phase by phase. Features are responsible for
checking that a candidate column belongs to their target biome.

The current provider is synchronous because `ServerChunkProvider::generate` is
synchronous. Expensive future structure or file generation should move behind
an asynchronous generation pipeline rather than hiding blocking work in a
feature.

## Default biomes

The vanilla server biome pack currently registers:

| Biome | Main surface | Climate role | Features |
| --- | --- | --- | --- |
| Plains | grass/dirt | temperate, medium humidity | caves, ores, sparse oak trees |
| Oak Forest | grass/dirt | temperate, high humidity | caves, ores, dense oak trees |
| Dry Desert | sand | hot and dry | caves, ores, cacti |
| Frozen Tundra | snow/dirt | cold, medium humidity | caves, ores, packed-ice patches |
| Rocky Peaks | stone/gravel | high continentalness | caves, ores, boulders |

These definitions are separate mods. A server can omit one, replace its
definition while keeping its generated ID, or compose a different definition
set with a custom selector.

## Default feature mods

| Mod | Registered feature IDs | Phase |
| --- | --- | --- |
| `server-biome-feature-caves-vanilla-mod` | `demo:caves` | Carving |
| `server-biome-feature-ores-vanilla-mod` | `demo:diamond-ores` | Underground |
| `server-biome-feature-oak-trees-vanilla-mod` | sparse and dense oak trees | Decoration |
| `server-biome-feature-cacti-vanilla-mod` | `demo:cacti` | Decoration |
| `server-biome-feature-ice-patches-vanilla-mod` | `demo:ice-patches` | Surface |
| `server-biome-feature-boulders-vanilla-mod` | `demo:rock-boulders` | Decoration |

The labels "sparse" and "dense" are two registered configurations of the same
tree algorithm. A feature mod may register several IDs when the behavior is
shared but its placement policy differs.

## Adding a biome

### 1. Add the identity contributor

```toml
[package]
name = "biome-lavender-fields"

[package.metadata.mod]
entry = "BiomeLavenderFieldsMod"

[package.metadata.mod.dependencies]
init = []
run = []
ownership = []

[package.metadata.biome]
id = "example:lavender-fields"
```

The entry can be an empty contributor mod. Add it to `biomes.toml` and
recompose so `BiomeId::LavenderFields` exists.

### 2. Add optional features

Register each independently useful generator behind an ID:

```rust
pub const LAVENDER_PATCHES: &str = "example:lavender-patches";

registry.register_feature(
    BiomeFeatureId::new(LAVENDER_PATCHES),
    LavenderPatchesFeature,
)?;
```

Give the feature a correct phase and conservative vertical range. Depend on
the feature mod from the runtime biome definition so Patchwork initializes it
first.

### 3. Register the server definition

```rust
registry.register_biome(BiomeDefinition {
    id: BiomeId::LavenderFields,
    name: "Lavender Fields",
    climate: BiomeClimate {
        temperature: 0.62,
        humidity: 0.56,
        continentalness: 0.40,
        has_precipitation: true,
        downfall: 0.48,
    },
    terrain: BiomeTerrain {
        base_height: 6.0,
        height_variation: 2.0,
        detail_variation: 0.8,
        surface: BlockId::Grass,
        subsurface: BlockId::Dirt,
        underground: BlockId::Stone,
        subsurface_depth: 3,
    },
    visuals: BiomeVisuals {
        sky_color: [0.55, 0.70, 0.96],
        fog_color: [0.76, 0.78, 0.92],
        water_color: [0.22, 0.42, 0.72],
        grass_tint: [0.52, 0.70, 0.38],
        foliage_tint: [0.44, 0.64, 0.34],
    },
    features: vec![
        caves_feature_id(),
        ores_feature_id(),
        BiomeFeatureId::new(LAVENDER_PATCHES),
    ],
})?;
```

This is normal Rust. The definition may call constructors, use constants from
other mods, or build feature lists conditionally from compile-time feature
choices. There is no biome JSON loader in the current architecture.

### 4. Select it in a server feature pack

Add the runtime definition and feature mods to a server modpack. Do not add
custom server world policy to `server-base.toml`.

The vanilla climate selector automatically considers every registered
definition. A custom selector may require an explicit mapping for the new ID.

## Replacing larger pieces

Use the narrowest replacement that matches the desired change:

- different climate layout: provide `server-biome-selection-api`;
- different cave algorithm: register another feature ID and use it in chosen
  definitions;
- different terrain strata using the same biomes: provide another primary
  chunk provider;
- saved biome map: selector reads a non-blocking cached map;
- per-player terrain: inspect `ChunkViewer`, and ensure routing creates separate
  world scopes;
- no biomes: omit `server-biomes-vanilla.toml`, remove or ignore
  `server-chunk-provider-biomes-mod`, and select another primary provider.

`server-biomes-vanilla.toml` owns only the vanilla biome content and selection
policy. `server-vanilla.toml` selects the generic runtime registry and the
biome-aware primary chunk provider. This keeps biome definitions reusable while
making the server's provider choice explicit in the server policy pack.

Do not add a special case to `server-chunk-world-dynamic-impl`. The world cache
already treats generation as a provider concern.

## Required block textures

The biome block mods already declare namespaced asset paths. Add these files to
their owning mods:

```text
mods/block-sand/assets/sand.png
mods/block-snow/assets/snow.png
mods/block-gravel/assets/gravel.png
mods/block-packed-ice/assets/packed_ice.png
mods/block-oak-leaves/assets/oak_leaves.png
mods/block-oak-log/assets/oak_log_side.png
mods/block-oak-log/assets/oak_log_top.png
mods/block-cactus/assets/cactus_side.png
mods/block-cactus/assets/cactus_top.png
mods/block-cactus/assets/cactus_bottom.png
```

Patchwork copies each directory to `assets/<mod-name>/`, which matches the
paths in each `BlockRenderInfo`. Until the PNG files are present, those texture
loads are expected to fail at runtime; the Rust composition still compiles.

## Current limits

The biome system intentionally does not yet implement every field found in a
large production voxel game:

- no biome map is sent to the client;
- visual colors are not applied by the renderer;
- no water or precipitation simulation;
- no biome-specific music or audio contract;
- no creature spawn tables;
- no generated structure registry;
- registries are populated at startup rather than changed during play;
- generation remains synchronous;
- features can write only the current chunk and must use deterministic clipped
  generation for cross-boundary shapes.

These should become separate APIs and feature mods when implemented. They do
not belong as extra responsibilities inside the current climate selector or
chunk provider.
