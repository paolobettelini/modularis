# Animated grass and wind

Short grass is a complete Patchwork feature rather than a branch inside the
normal chunk renderer. The implementation ports the core ideas from the
included Fabric Grassier Grass mod into the demo's Bevy architecture:

- deterministic blades generated per block;
- tapered, segmented ribbons;
- distance-dependent density and geometry LOD;
- chunk-local cached meshes;
- bounded rebuild work;
- smooth broad gusts and per-blade flutter in a vertex shader;
- configurable height, width, density, color, range, and wind.

Minecraft, Fabric, Sodium, Iris, OpenGL buffer, and mixin classes are not copied
into the runtime. Their responsibilities are mapped to small Patchwork
contracts and Bevy systems.

## Feature split

```text
block-short-grass
        │
        ├── server-biome-feature-short-grass-vanilla-mod
        │       writes logical blocks during world generation
        │
        └── client-grass-mesh-fabric-style-impl
                ├── provides client-grass-mesh-api
                └── consumes client-grass-tint-api
                              ^
                              │
                   client-grass-tint-vanilla-mod

client-grass-render-bevy-impl
        ├── client-grass-mesh-api
        ├── client-grass-settings-api
        ├── client-wind-api
        └── client-grass-interaction-api
                              ^
                              │
          client-grass-interaction-state-mod
                              ^
                              │
       client-grass-player-contact-vanilla-mod
```

`client-grass.toml` selects the client-side settings contributors, typed
settings adapter, vanilla wind provider, Fabric-style mesh provider, and Bevy
renderer. Removing that modpack leaves the logical block and server generation
valid, but the block has no visible client blades.

This separation permits useful replacements:

- a low-end client can provide a billboard or instanced mesh implementation;
- another wind provider can synchronize weather from the server;
- a different renderer can use compute shaders without changing world data;
- a custom server can place short grass with another biome feature;
- a server can omit short grass entirely.

## Logical block

`block-short-grass` contributes `demo:short_grass`. It is non-solid,
non-opaque, and not air. Its JSON model has one selection element but no visible
faces.

The empty visible model is deliberate:

- normal voxel-model shape resolution supplies the selection shape;
- collision ignores it because the block is non-solid;
- the ordinary chunk mesher emits no duplicate plant quads;
- the dedicated grass renderer owns presentation.

Logical identity therefore does not depend on a particular grass renderer.

## Geometry provider

`client-grass-mesh-api` exposes `GrassMeshService`. Its selected function
receives:

- one cached `Chunk`;
- a typed `ClientGrassSettings` snapshot;
- horizontal distance from the streaming focus.
- the current generated `Dimension`.

It returns plain vertex and index arrays plus a blade count. It does not spawn
entities, load shaders, inspect cameras, or mutate the chunk cache.

`client-grass-mesh-fabric-style-impl` is the current provider. For every
`BlockId::ShortGrass`, it:

1. derives a stable random stream from world block coordinates and blade index;
2. applies configured sparsity;
3. chooses a position, orientation, height, width, and animation phase;
4. creates two crossed tapered ribbons;
5. reduces blade count and ribbon segments at distance.

For every plant it also looks up the supporting block and asks
`client-grass-tint-api` for a color using `(Dimension, BlockId)`. The vanilla
provider gives Overworld grass a muted natural green, maps Nether surfaces to
crimson, warped, soul, basalt, or netherrack palettes, and uses brighter or
golden colors for Aether surfaces. Color policy is therefore replaceable
without replacing geometry generation or the renderer.

The stable seed prevents grass from changing shape after chunk rebuilds.
World-generation order and frame timing do not affect the result.

The current LOD density multipliers are:

| Horizontal distance | Density | Ribbon segments |
| --- | ---: | ---: |
| up to 32 blocks | 100% | 3 |
| 32–80 blocks | 60% | 2 |
| 80–128 blocks | 35% | 1 |
| over 128 blocks | 20% | 1 |

Disabling grass LOD uses full density and three segments at every distance.

## Bevy renderer

`client-grass-render-bevy-impl` provides `client-grass-render-api`. It listens
to the same cache lifecycle as normal chunk rendering:

- `ClientChunkAvailable`;
- `ClientChunkChanged`;
- `ChunkUnload`;
- `ChunkStreamingFocus` changes;
- geometry-affecting grass setting changes.

It keeps a deduplicated pending set and processes at most two grass chunks per
frame, nearest first. Only four vertical chunk layers above or below the focus
are considered, matching the useful bound from the Fabric implementation.
Horizontal range comes from the grass settings.

Each non-empty result becomes one Bevy mesh entity at the chunk origin. A
single shared custom material is reused by every grass chunk. Each entity also
gets a conservative explicit culling bound, expanded for shader-side tip
movement; this avoids wind-displaced blades popping at the edge of the camera
without disabling frustum culling.

The renderer and the normal chunk renderer intentionally remain separate.
Grass can use a custom shader and rebuild budget without complicating the
general JSON voxel-model mesher.

## GPU wind

`client-wind-api` owns only:

```rust
pub struct ClientWind {
    pub direction: Vec2,
    pub intensity: f32,
}
```

The selected `client-wind-grass-settings-vanilla-mod` derives this state from
the grass settings and Bevy time. It combines:

- a broad direction drift;
- a smaller secondary direction wave;
- a broad intensity pulse;
- a secondary intensity pulse.

The Bevy grass material sends wind and appearance values to
`grass.wgsl`. The vertex stage anchors every blade at its root, bends its upper
vertices along the world-space wind direction, and adds faster perpendicular
flutter. Geometry is not rebuilt while wind changes.

`client-wind-api` is intentionally not named after grass. Future leaves,
particles, cloth, or weather presentation can consume the same neutral client
wind resource. A network-backed weather provider can replace the local vanilla
provider.

## Contact deformation

Touch deformation is presentation, not solid collision. Short grass remains a
non-solid block and does not stop the player.

`client-grass-interaction-api` stores named `GrassInteractionSource` values.
Each source describes an oriented capsule with a world-space center, axis,
half-length, radius, and strength. A volume rather than a point keeps contact
correct across the whole player hitbox, including scaled players and players
whose gravity changes their up direction.

`client-grass-player-contact-vanilla-mod` publishes the local player after
movement. `client-grass-network-player-contact-vanilla-mod` independently
publishes rendered network players, using their replicated scale and gravity.
Removing either mod disables only that source family.

The current grass renderer selects the eight sources nearest to the streaming
focus and sends them to the GPU. The vertex shader measures distance from each
blade vertex to each capsule, bends nearby blade tips away, and lowers them
slightly without rebuilding chunk meshes. Eight is a renderer budget, not an
API restriction: another rendering provider can support a different number or
representation.

Other object features should publish their own named source through the same
API instead of adding entity queries to the grass renderer. Removing the
player-contact mod disables that behavior while leaving wind and rendering
intact.

## Settings submenu

Settings contributors may declare an optional section:

```toml
[package.metadata.setting]
id = "grass.wind_speed"
label = "Wind speed"
type = "f32"
input = "f32"
default = 1.0
min = 0.0
max = 5.0
section = "graphics/grass"
section_label = "Grass"
```

Slash-separated section IDs describe a hierarchy. The generated settings
registry preserves the path and derives its parent sections. The generic menu
therefore renders `Graphics` on the root settings page and `Grass` inside the
graphics page, both in the main menu and in the pause menu. Every generated
page has vertical scrolling; no grass-specific navigation code exists in the
generic menu implementation.

The current section contains:

| Setting | Default | Effect |
| --- | ---: | --- |
| Enabled | on | creates or removes grass meshes |
| Blades per block | 32 | source blade density |
| Sparsity | 0.10 | deterministic blade rejection |
| Blade height | 0.44 | base height in blocks |
| Height variation | 0.35 | per-blade height range |
| Blade width | 0.95 | ribbon width multiplier |
| Render radius | 96 | independent horizontal range in blocks |
| Distance LOD | on | density and segment reduction |
| Brightness | 1.0 | fragment color multiplier |
| Hue jitter | 8 degrees | stable per-blade color variation |
| Wind speed | 1.0 | base visual wind strength |
| Wind direction | 0 degrees | base XZ direction |
| Dynamic wind | on | enables smooth drift and pulses |
| Dynamic wind strength | 0.8 | pulse strength |
| Contact deformation | 1.0 | player/object bending amount; zero disables visible deformation |

The typed adapter clamps domain values before writing `ClientGrassSettings`.
Geometry settings queue chunk rebuilds. Appearance and wind settings only
update the shared material. Numeric `min` and `max` values are also part of the
generated settings contract: both typed input and step buttons are kept inside
the range, and programmatic writes are clamped by `SettingsStore`.

The grass material uses a compact custom vertex interface and performs its wind
deformation in its own forward pass. It opts out of Bevy's standard material
prepass and shadow pass because those passes use the standard mesh vertex
interface. Lighting or grass shadows should be supplied by a dedicated,
grass-compatible rendering mod rather than enabling the standard PBR passes on
this material.

## Server biome placement

`server-biome-feature-short-grass-vanilla-mod` registers sparse and dense
feature IDs. The feature:

- runs in `Decoration`;
- declares a vertical range of exactly one block above the surface;
- uses the world-generation hash supplied by `BiomeFeatureContext`;
- writes only when the target position is air.

Plains select dense placement. Oak and birch forests select sparse placement.
Nether Wastes, Soul Sand Valley, and Basalt Deltas select sparse placement;
Crimson and Warped Forests select dense placement. Aether Highlands and Golden
Grove select dense placement, while Crystal Peaks select sparse placement.
Desert, badlands, tundra, and rocky Overworld definitions do not select it.

The feature is deliberately surface-agnostic. Selection belongs to each biome
definition, while the client tint provider interprets the supporting block and
dimension. A custom biome can reuse either feature ID, register a third density,
or implement a different placement algorithm.

## Extending the feature

Prefer one small mod for each independent addition:

- persistent interaction trails: a trail-field API plus a shader-consuming
  mod;
- weather authority: server state, packet contributor, client receiver, and a
  `client-wind-api` provider;
- snow-covered blades: block/environment query contract plus a material stage;
- flowers: a separate biome feature and presentation provider;
- grass particles: an event-driven client effect mod;
- another blade style: a new `client-grass-mesh-api` provider;
- exact biome-colored grass: transmit biome samples and provide a replacement
  `client-grass-tint-api` implementation.

Do not add these policies to `block-short-grass`. The block owns identity and
logical properties, not every behavior that may involve grass.
