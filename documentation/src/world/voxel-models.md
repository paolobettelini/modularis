# JSON voxel models and textures

Block and item geometry is data-driven. Content mods export Minecraft-style
JSON models and texture files, while client provider mods decide how those
files are loaded, baked, meshed, and presented.

This keeps three concerns independent:

- a block or item mod declares which model belongs to its domain ID;
- template mods provide reusable model inheritance without gameplay code;
- client mods choose the model source, chunk mesher, and UI renderer.

The server does not parse presentation assets. It still composes the content
mods because block and item identities are shared, but model loading belongs to
the client provider selected in `client.toml`.

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
| `voxel-models-lib` | Engine-neutral JSON parsing, inheritance, texture-variable resolution, and quad baking |
| `voxel-model-api` | Replaceable runtime service for resolving a model ID into baked quads |
| `client-voxel-model-assets-fs-impl` | Filesystem provider for Patchwork's composed asset layout, with result caching |
| `block-render-api` | Associates a block ID with an optional model ID |
| `item-render-api` | Associates an item ID with an optional model ID |
| `client-chunk-mesh-voxel-models-impl` | Builds chunk geometry from baked block-model quads |
| `client-item-model-ui-mod` | Uses item models as the default inventory presentation |

`voxel-model-api` is intentionally not tied to filesystem loading. Another
client can provide models from an archive, embedded bytes, a development hot
reload service, or a resource-pack stack without changing block contributors
or the chunk renderer.

## Data flow

```text
block/item contributor
  -> generated registry render_info(id)
  -> VoxelModelService.bake(namespaced model ID)
  -> JSON parent and texture resolution
  -> baked texture-tagged quads
  -> chunk mesher or item presentation
  -> Bevy assets and entities
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

Rendering shape and collision shape remain separate contracts. The current
collision provider still treats every `solid` block as a full-block AABB, so
the new partial visual models do not yet imply stair- or cauldron-shaped
collision. A future collision-shape API can be added independently.

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
`VoxelModelService`. Select exactly one provider in the client modpack.

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
7. keep model geometry, collision policy, and gameplay behavior in separate
   contracts.
