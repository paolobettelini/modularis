# Graphics, lighting, sky, and outlines

Graphics policy is separated from world state and client bootstrap.

## Chunk material inputs

Chunk meshes provide:

- positions;
- normals;
- UVs;
- vertex colors;
- indices;
- optional texture path.

The renderer uses a white base material, so vertex colors can multiply
brightness and textures retain their authored color.

## Vertex lighting pipeline

`ChunkVertexLightingPipeline` is append-only and supports:

- face brightness stages;
- ambient occlusion stages.

Brightness is multiplicative:

```text
final = product(face stages) * product(AO stages)
```

Each stage clamps to `0..=1`.

The mesher takes one snapshot per chunk. It does not lock the stage registry for
every vertex.

With no registered stages, brightness is white (`1.0`).

## Face shading

The vanilla face stage assigns:

| Face | Brightness |
| --- | --- |
| Top | `1.00` |
| South | `0.88` |
| East/West | `0.78` |
| North | `0.68` |
| Bottom | `0.55` |

This creates a simple voxel-style directional contrast without expensive
lighting calculations.

## Voxel ambient occlusion

For every face vertex, the mesher samples:

- side A;
- side B;
- diagonal corner.

When both sides are opaque, the corner is maximally occluded. Otherwise the
number of occupied samples selects a brightness:

```text
0 -> 1.00
1 -> 0.84
2 -> 0.69
3 -> 0.54
```

The mesher chooses one of two quad diagonals based on opposite vertex brightness
sums, reducing visible AO interpolation seams.

AO may sample chunks across two or three axes, which is why mesh invalidation
uses all 26 neighbors.

## Sun state and authority

Shared `SunSettings` contains:

```rust
position: [f32; 3]
illuminance: f32
color: [f32; 3]
```

Server layers:

- `server-sun-state-mod`: neutral optional state;
- `server-sun-network-sync-mod`: join and runtime synchronization;
- `server-sun-vanilla-mod`: selected initial value.

The vanilla default is policy. Other server systems can change the sun at
runtime without knowing Bevy rendering.

Client layers:

- state resource and changed message;
- network receive;
- directional light renderer;
- independent shadow-enabling mod;
- ambient fill;
- sun disc.

## Directional light

The renderer converts normalized negative sun position into a light direction.
It preserves the current `shadows_enabled` value when applying changed settings,
allowing the shadow mod to remain independent.

The light uses server-provided color and illuminance.

## Ambient fill

The ambient mod uses brightness `360` and a mostly neutral color with a `35%`
sun tint.

This keeps shadows readable while allowing an extreme server sun color, such as
red, to affect shadowed areas too.

## Sun disc

The disc is:

- an unlit sphere;
- positioned relative to the camera at fixed distance;
- colored directly from sun settings;
- excluded from shadow casting and receiving.

It gives the light direction a visible representation without coupling it to
the directional light entity.

## Sky

Dimension synchronization sends a sky color. `client-sky-bevy-mod` maps
`ClientSkyColor` to Bevy `ClearColor`.

Sky state is independent from sun state:

- a dimension chooses sky background;
- server sun chooses light direction/intensity/color.

## Shadow policy

`client-sun-shadows-vanilla-mod` only sets `shadows_enabled` on the tagged sun
light. Removing it keeps directional lighting but disables shadows.

This is a good example of a small composable graphics enhancement.

## Block outlines

The outline API defines:

```rust
SetClientBlockOutline {
    owner,
    block: Option<BlockPos>,
    shape: BlockShape,
    style,
}
```

`owner` lets independent mods maintain separate outlines.

The pipeline is:

```text
Collect -> Apply -> Draw
```

The Bevy provider creates thin, unlit edge meshes around every local AABB in
`shape`. Full cubes use twelve edges; stairs, anvils, cauldrons, and other
element models follow their box union. It does not create a screen-space
cursor; the crosshair remains owned entirely by `client-crosshair-bevy-mod`.

The looked-block vanilla mod performs a reach-limited shape raycast and owns
the key `vanilla:looked-block`.

The current client composition selects the renderer in `client.toml` and the
looked-block policy in `client-vanilla.toml`. Both are required: the provider
draws owner-keyed outline state, while the policy performs the raycast and
publishes `vanilla:looked-block` updates.

## Adding a lighting stage

Create a mod that depends on `ClientChunkVertexLightingApi` and registers a pure
function in `init`.

Example:

```rust
fn softer_bottom(face: BlockFace) -> f32 {
    match face {
        BlockFace::Bottom => 0.75,
        _ => 1.0,
    }
}
```

Remember that selected stages multiply. A replacement for vanilla face shading
should remove that mod rather than register a second full face policy.

## Replacing the renderer

A more advanced renderer may add:

- greedy meshes;
- texture arrays/atlases;
- transparent block passes;
- normal maps;
- cascaded shadow tuning;
- fog;
- GPU-driven chunk culling;
- scoped skyboxes.

It should keep streaming and server protocol independent unless its data needs
truly require a new contract.
