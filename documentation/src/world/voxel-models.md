# JSON voxel models and textures

Block and item geometry is data-driven. Content mods export Minecraft-style
JSON models and texture files, while provider mods decide how those files are
loaded and baked and separate consumers decide how geometry is used.

This keeps three concerns independent:

- a block or item mod declares which model belongs to its domain ID;
- template mods provide reusable model inheritance without gameplay code;
- a model provider exposes resolved quads and element bounds;
- client mods choose the chunk mesher and UI renderer;
- collision, raycast, and outline mods consume a replaceable block-shape API.

The active server also resolves block-model element bounds, but it never owns
texture loading or rendering. This is a composition choice rather than a core
rule: another server can provide authored collision shapes without loading JSON
models at all.

## Asset layout

An asset-owning content mod uses this layout:

```text
mods/block-marble/
├── Cargo.toml
├── src/lib.rs
└── assets/
    ├── models/
    │   └── block/
    │       └── marble.json
    └── textures/
        └── block/
            └── marble.png
```

Item models use `assets/models/item/` and item textures use
`assets/textures/item/`.

Patchwork copies this tree to:

```text
build-client/client/assets/block-marble/
```

The mod crate name is therefore the resource namespace. The model ID
`block-marble:block/marble` resolves to:

```text
assets/block-marble/models/block/marble.json
```

and the texture ID `block-marble:block/marble` resolves to:

```text
assets/block-marble/textures/block/marble.png
```

Resource IDs should always be namespaced. A bare `block/marble` would use the
default `minecraft` namespace and would not point at the owning Patchwork mod.

## Runtime parts

The model pipeline is split across small crates.

| Crate | Responsibility |
| --- | --- |
| `voxel-models-lib` | Engine-neutral JSON parsing, inheritance, texture-variable resolution, quad baking, and element-bound baking |
| `voxel-model-api` | Replaceable runtime service for resolving a model ID into baked quads and boxes |
| `voxel-model-assets-fs-impl` | Generic filesystem provider for Patchwork's composed asset layout, with result caching |
| `block-shape-api` | Replaceable block-instance-to-local-AABB-union contract |
| `block-shape-voxel-model-impl` | Derives one local AABB from each resolved JSON `element` |
| `block-render-api` | Associates a block ID with an optional model ID |
| `item-render-api` | Associates an item ID with an optional model ID |
| `client-chunk-mesh-voxel-models-impl` | Builds chunk geometry from baked block-model quads |
| `client-item-model-ui-mod` | Uses item models as the default inventory presentation |

`voxel-model-api` is intentionally not tied to filesystem loading. Another
composition can provide models from an archive, embedded bytes, a development
hot-reload service, or a resource-pack stack without changing block
contributors or consumers.

## Data flow

```text
block/item contributor
  -> generated registry render_info(id)
  -> VoxelModelService.load(namespaced model ID)
  -> JSON parent and texture resolution
  -> cached baked quads + cached element boxes
       |                         |
       -> client mesh/UI         -> BlockShapeService
                                  -> collision/raycast/outline
```

Successful and failed resolutions are cached. A bad model therefore reports a
stable error instead of repeatedly reading and parsing the same files during
chunk remeshing.

## Reusable template mods

`voxel-model-block-templates-mod` contains generic block parents such as:

- `block/block`;
- `block/cube`;
- `block/cube_all`;
- `block/cube_column`;
- `block/stairs`.

`voxel-model-item-templates-mod` contains `item/generated` and
`item/handheld`. `voxel-model-anvil-template-mod` contains the reusable anvil
geometry and itself depends on the generic block template mod.

These are real Patchwork mods even though their Rust entry types do no runtime
work. A child content mod declares the template mod as a formal `init`
dependency and receives it in `init`:

```toml
[package.metadata.mod.dependencies]
init = ["voxel-model-block-templates-mod"]

[dependencies]
voxel-model-block-templates-mod = {
    path = "../voxel-model-block-templates-mod"
}
```

```rust
pub fn init(
    _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
) -> Self {
    Self
}
```

The argument is deliberately unused at runtime. It makes the asset inheritance
edge visible to Patchwork, so a composition cannot select the child model while
silently omitting its parent assets.

## Defining a cube block

The block crate keeps logical properties in Rust and points presentation at a
model:

```rust
impl BlockRender for MarbleBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-marble:block/marble"),
        textures: None,
    };
}
```

Its JSON supplies the texture variable expected by the shared parent:

```json
{
  "parent": "voxel-model-block-templates-mod:block/cube_all",
  "textures": {
    "all": "block-marble:block/marble"
  }
}
```

For different faces, inherit `block/cube` and define `down`, `up`, `north`,
`south`, `west`, and `east`. Geometry is no longer hardcoded in the block
contributor.

## Defining a non-cube block

A non-cube model is just another JSON model containing `elements`. The anvil,
oak stairs, and cauldron use the same block render API as a full cube. The
mesher does not contain branches for those IDs.

Use `cullface` only when a face lies on the corresponding full block boundary.
Internal faces and partial-shape faces normally omit it. The client mesher
culls a tagged face when the neighboring block is opaque.

Visual geometry and physical geometry remain separate contracts even though
the default composition derives both from the same JSON elements.
`block-shape-voxel-model-impl` converts every resolved element to a normalized
local AABB. A stair therefore becomes a union of boxes, while an anvil or
cauldron follows its own element layout. Element rotations are represented by
their smallest enclosing AABB; this is conservative collision, not an oriented
box solver.

`BlockShape` precalculates the boundary edges of the complete AABB union when
the shape is created. Coplanar subdivisions and internal contacts are removed:
eight half-size boxes arranged as one cube still produce only the twelve outer
edges. Consecutive collinear segments are merged. The cached result is shared
by cheap `BlockShape` clones and can be consumed by outline renderers without
rebuilding topology every frame.

`BlockInfo::solid` remains an independent gameplay policy. Client/server
collision ignore the shape for non-solid blocks, while selection raycasts and
outlines can still use the geometry. A custom composition can replace only the
shape provider without replacing rendering or block identity.

## Defining an item model

Item contributors export `ITEM_RENDER_INFO` in addition to `ITEM_INFO`:

```rust
impl ItemRender for StickItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-stick:item/stick"),
    };
}

pub const ITEM_RENDER_INFO: ItemRenderInfo =
    <StickItem as ItemRender>::RENDER;
```

`item-registry-codegen` collects both constants and generates
`render_info(ItemId)`. Item identity code does not need to know which client
presentation provider is selected.

A flat generated item can inherit the shared template:

```json
{
  "parent": "voxel-model-item-templates-mod:item/handheld",
  "textures": {
    "layer0": "item-stick:item/stick"
  }
}
```

A block item may inherit its block model directly. This expresses that both
presentations share geometry while retaining separate item and block IDs.

The current Bevy inventory adapter uses the first resolved model texture as a
2D slot image. Generated items therefore display their `layer0` texture, while
block items get a texture-based preview. Full isometric model rendering in UI
can be implemented later as another item presentation mod without changing
item contributors or registries. Explicit `ItemFavicon` instance metadata is
still supported as an override and is owned by its separate decorator mod.

## Lighting and mesh output

The model mesher preserves the existing chunk pipeline:

- quads are grouped by resolved texture;
- model normals select the face-lighting direction;
- `shade: false` bypasses directional shading;
- light emission raises the minimum vertex brightness;
- the ambient-occlusion pipeline samples nearby opaque blocks;
- the quad diagonal follows the AO gradient;
- uniform-air and fully-hidden uniform-chunk fast paths remain active.

The renderer still owns Bevy materials, texture loading, mesh entities, and
per-frame remesh budgets. Model parsing does not own those concerns.

## Adding another model source or renderer

To replace filesystem loading, implement `VoxelModelApi` and insert a
`VoxelModelService`. Select exactly one provider in every composition that
needs model data.

To author physical geometry independently, implement `BlockShapeApi` and
insert a `BlockShapeService`. The service returns local-space AABB unions for a
`BlockInstance`, not only a block ID. The default provider ignores metadata,
but a future orientation or open/closed metadata provider can change geometry
without breaking the API. Collision, precise raycast, placement occupancy
checks, and outlines do not need to know where those boxes came from.

To replace chunk geometry generation, implement `ChunkMeshApi` and insert a
`ChunkMeshService`. A greedy or asynchronous mesher can consume the same baked
models.

To replace inventory presentation, add another mod that listens to
`InventorySlotVisualCreated`. It can render a full 3D preview, use an atlas, or
apply model display transforms. It should not change the authoritative item
instance.

## Validation checklist

When adding a model-owning mod:

1. use namespaced model and texture IDs;
2. declare formal dependencies for every cross-mod parent;
3. add the contributor and template to the appropriate modpack;
4. run Patchwork composition so registries and assets are regenerated;
5. check the composed `assets/<mod-name>/` tree;
6. check both client and server compositions when shared IDs changed;
7. test element-derived collision and selection for partial models;
8. keep model geometry, shape provisioning, solidity policy, and gameplay
   behavior in separate contracts.
