# Biomes and world-generation features

Biomes are available in every current dimension. The system is assembled from
independent layers:

```text
generated BiomeId contributors
             │
dimension-scoped runtime definitions + feature registry
             │
viewer/instance-aware biome selector
             │
shared request-local biome sampler
             │
Overworld, Nether, or Aether geometry provider
```

The split is intentional. A biome ID is domain identity, a definition is
server content, selection is policy, and a tree or cave is an optional feature.
The sampler only shares lookup, caching, terrain blending, and phased feature
dispatch. It does not decide whether terrain is a heightmap, Nether ground, or
floating islands.

## Generated biome identity

Every biome identity contributor is a small mod with one namespaced ID:

```toml
[package.metadata.biome]
id = "example:lavender-fields"
```

`biome-registry-codegen` scans the composed project and generates `BiomeId`,
`ALL_BIOMES`, string conversion, and lookup. Only the ID belongs in Cargo
metadata. Dimension, climate, terrain, visuals, and features stay in Rust code.

The identity and server definition are separate mods:

```text
biome-forest
    contributes demo:forest to codegen

server-biome-forest-vanilla-mod
    registers the demo server's runtime definition
```

This allows a protocol or client to know the ID without importing server world
policy. Duplicate IDs and duplicate generated Rust variants fail during
composition.

Identity packs are split by dimension:

- `biomes-overworld.toml`;
- `biomes-nether.toml`;
- `biomes-aether.toml`;
- `biomes.toml`, which imports all three for the demo composition.

A custom server may import only the dimensions it supports.

## Runtime definitions

`server-biome-api` owns the runtime contract:

```rust
pub struct BiomeDefinition {
    pub id: BiomeId,
    pub dimension: Dimension,
    pub name: &'static str,
    pub climate: BiomeClimate,
    pub terrain: BiomeTerrain,
    pub visuals: BiomeVisuals,
    pub features: Vec<BiomeFeatureId>,
}
```

`dimension` prevents the Nether selector from considering Overworld or Aether
definitions. `ServerBiomeRegistry::definitions_for_dimension` returns a sorted
copy containing only the requested dimension.

### Climate

`BiomeClimate` stores target temperature, humidity, continentalness,
precipitation support, and downfall. These values are hints for selectors, not
fixed universal ranges.

The vanilla selector combines two signals:

- an independent broad Perlin field per biome, giving every selected biome a
  similar chance to own regions;
- a smaller climate-distance penalty, preserving hot, cold, wet, and dry
  character without letting central climate points dominate the map.

The region frequency is intentionally broad enough for readable areas while
still showing several biomes during local flight. The current multiplier is
`4.0`, half the previous test value, so regions are about twice as wide along
each horizontal axis.

A replacement selector may use ranges, latitude, Voronoi regions, player
state, saved maps, or no climate data.

### Terrain

`BiomeTerrain` stores base height, broad and detail variation, surface,
subsurface and underground blocks, and subsurface depth.

Providers interpret these fields. The Overworld uses them for a normal
heightmap, the Nether for low terrain that continues through negative chunk
layers, and the Aether for island height and material layers. A file-backed
provider may ignore all height fields while still using biome IDs and
features.

### Visuals

`BiomeVisuals` records sky, fog, water, grass, and foliage colors. They are an
extension point for client synchronization. The current chunk protocol does
not transmit biome maps, so the renderer does not apply these values yet.

### Feature list

A definition stores ordered feature IDs rather than concrete implementations:

```rust
features: vec![
    caves_feature_id(),
    ores_feature_id(),
    dense_oak_trees_feature_id(),
]
```

Feature mods register those IDs separately. Missing implementations and
duplicate registrations are rejected by registry validation.

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

Public phases provide deterministic cooperation between unrelated mods:

```text
Carving -> Underground -> Surface -> Decoration -> Finalization
```

Examples include caves in `Carving`, ores in `Underground`, ice in `Surface`,
and trees, boulders, glowstone clusters, and crystal spires in `Decoration`.

Each feature also declares a conservative vertical range. Providers can then
return uniform air or underground chunks without running irrelevant feature
code. A bound must never be narrower than the feature's possible writes.

`BiomeFeatureContext` exposes the generation request, target biome, world seed,
surface and biome sampling callbacks, current-chunk reads, clipped writes, and
deterministic position hashing. Cross-chunk shapes inspect neighboring anchors
but only write the current chunk. This makes output independent of generation
order.

## Selection and request scope

Selection is an exclusive API:

```rust
pub trait ServerBiomeSelector {
    fn select(
        &self,
        request: &BiomeSelectionRequest<'_>,
        definitions: &[BiomeDefinition],
    ) -> Option<BiomeId>;
}
```

The request contains the original `ChunkGenerationRequest`, world X/Z,
viewer, and world instance. A custom selector can therefore vary by instance
or player. If it varies by player, chunk routing and edit identity must use a
matching scope; otherwise two different logical worlds would share edits.

The vanilla selector derives a stable noise seed from `ServerWorldSeed`, its
own namespace, and `WorldInstanceId`, then uses stable ID tie-breaking. Only an
Overworld definition set containing `BiomeId::Plains` receives the small safe
Plains area around spawn. Nether and Aether selections cannot accidentally
select Plains because definitions are filtered first.

## Shared biome sampler

`server-biome-sampling-api` is a pure support crate used by all three current
providers. A `ServerBiomeSampler`:

- filters definitions to one `Dimension`;
- caches `(x, z) -> BiomeId` per chunk request;
- resolves definitions and terrain data;
- blends neighboring terrain parameters;
- finds active feature implementations;
- tests feature vertical intersections;
- dispatches features in public phase order.

It does not register a provider and does not create blocks. This is important
for Patchwork replacement: a custom island or saved-world provider can reuse
the sampler, while a server with no biomes can omit it.

## Dimension-specific providers

### Overworld

`server-chunk-provider-biomes-mod` creates a continuous heightmap. It blends
height parameters across biome boundaries, applies biome-specific strata,
keeps a safe spawn blend, and runs features. Uniform sky and deep underground
chunks use compact one-entry palettes where possible.

### Nether

`server-chunk-provider-nether-mod` keeps its own Nether geometry and provider
ID. It derives surface shape and material strata from Nether biome terrain and
applies Nether features. It does not treat world Y zero as a floor: doing so in
an unbounded vertical coordinate system would create a bedrock wall through the
middle of the world. Chunks above every relevant surface and feature are
returned as uniform air. A bedrock boundary, if desired, belongs in a separate
feature or bounded provider.

### Aether

`server-chunk-provider-aether-mod` generates sparse floating-island columns.
Biome terrain controls island material and height character, while the provider
owns island presence and thickness. Columns outside islands remain air and
features run only where an island exists. A small spawn island remains
guaranteed.

The three providers are independent implementation mods. Sharing a sampler is
not the same as merging their terrain policy.

## Default biomes

### Overworld

| Biome | Main layers | Main features |
| --- | --- | --- |
| Plains | grass, dirt, stone | caves, ores, sparse oak trees |
| Oak Forest | grass, dirt, stone | caves, ores, dense oak trees |
| Birch Forest | grass, dirt, stone | caves, ores, birch trees |
| Dry Desert | sand, stone | caves, ores, cacti |
| Red Badlands | red sand, terracotta, stone | caves, ores, boulders |
| Frozen Tundra | snow, dirt, stone | caves, ores, packed-ice patches |
| Rocky Peaks | stone, gravel | caves, ores, boulders |

### Nether

| Biome | Main layers | Main features |
| --- | --- | --- |
| Nether Wastes | netherrack | caves, glowstone clusters |
| Soul Sand Valley | soul sand, soul soil, netherrack | caves |
| Crimson Forest | crimson nylium, netherrack | caves, glowstone clusters |
| Warped Forest | warped nylium, netherrack | caves, glowstone clusters |
| Basalt Deltas | basalt, blackstone, netherrack | caves, glowstone clusters |

### Aether

| Biome | Main layers | Main features |
| --- | --- | --- |
| Aether Highlands | grass, dirt, stone | sparse oak trees |
| Golden Grove | moss, dirt, calcite | dense oak trees, glowstone clusters |
| Crystal Peaks | calcite, stone | crystal spires |

Each definition is a separate mod. A server can omit or replace one without
editing a central biome switch.

## Default feature mods

| Mod | Purpose | Phase |
| --- | --- | --- |
| `server-biome-feature-caves-vanilla-mod` | deterministic caves | Carving |
| `server-biome-feature-ores-vanilla-mod` | diamond ore replacement | Underground |
| `server-biome-feature-oak-trees-vanilla-mod` | sparse/dense oak trees | Decoration |
| `server-biome-feature-birch-trees-vanilla-mod` | birch trees | Decoration |
| `server-biome-feature-cacti-vanilla-mod` | cacti | Decoration |
| `server-biome-feature-ice-patches-vanilla-mod` | packed ice | Surface |
| `server-biome-feature-boulders-vanilla-mod` | rock boulders | Decoration |
| `server-biome-feature-glowstone-clusters-vanilla-mod` | glowstone clusters | Decoration |
| `server-biome-feature-crystal-spires-vanilla-mod` | calcite/glowstone spires | Decoration |

A feature mod may register multiple IDs when one algorithm supports several
configurations.

## Adding a biome

### 1. Contribute identity

```toml
[package.metadata.biome]
id = "example:lavender-fields"
```

Add it to the appropriate dimension identity pack and recompose.

### 2. Add optional generation features

Create a separate mod implementing `ServerBiomeFeature`, register a namespaced
`BiomeFeatureId`, and declare its phase and conservative vertical range. The
definition mod should depend on the feature mod so it is initialized first.

### 3. Register the server definition

```rust
registry.register_biome(BiomeDefinition {
    id: BiomeId::LavenderFields,
    dimension: Dimension::Overworld,
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
    features: vec![BiomeFeatureId::new("example:lavender-patches")],
})?;
```

Definitions are normal Rust. There is no biome JSON loader in the current
architecture.

### 4. Select it in a server pack

Add the identity to a `biomes-<dimension>.toml` pack and the definition plus
features to a `server-biomes-<dimension>-*.toml` pack. Do not put custom world
policy in `server-base.toml`.

## Replacing larger pieces

Use the narrowest replacement:

- change one biome: replace only its definition mod;
- change a decoration: register a new feature and reference it;
- change placement policy: provide `server-biome-selection-api`;
- change geometry: provide a chunk provider while optionally reusing
  `server-biome-sampling-api`;
- use saved biomes: provide a selector backed by a non-blocking cache;
- use player-specific terrain: inspect `ChunkViewer` and use matching routing
  and edit scopes;
- use no biomes: omit biome packs and select another provider.

The vanilla packs are split into Overworld, Nether, and Aether content packs.
`server-biomes-vanilla.toml` imports them and selects the vanilla climate
policy. Provider selection stays in the server's world composition.

## Texture ownership

Every block mod owns its texture files and Patchwork copies them to
`assets/<mod-name>/`. The expanded biome set adds textures for birch logs and
leaves, red sand, terracotta, soul sand, soul soil, basalt, blackstone, crimson
and warped nylium, moss, and calcite. These files were copied from the local
vanilla texture pack into their owning mods; no provider has a central texture
directory.

## Current limits

- biome maps and visual colors are not synchronized to clients;
- there is no water or precipitation simulation;
- there are no biome-specific audio or creature-spawn contracts;
- there is no generated structure registry;
- registries are startup-time resources;
- generation is synchronous;
- features write only the current chunk and must use deterministic clipped
  generation for cross-boundary shapes.

These should become separate APIs and feature mods when implemented, not extra
responsibilities in the selector or chunk providers.
