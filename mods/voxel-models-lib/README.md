# voxel-minecraft-models

Rust library for parsing Minecraft Java Edition resource-pack JSON and turning resolved block/item models into voxel-friendly quads. The core crate has no engine dependency; the optional `bevy` feature converts texture-grouped geometry into Bevy 0.19 `Mesh` values.

## What is included

- legacy `models/block/*.json` and `models/item/*.json` parsing;
- model inheritance through `parent`;
- texture-variable resolution such as `#all` and `#side`;
- `elements`, faces, explicit/default UVs, face rotation, element rotation, `cullface`, tint indices and light emission;
- `blockstates/*.json` variants, weighted choices and multipart conditions;
- modern `items/*.json` model trees (`model`, `composite`, `condition`, `select`, `range_dispatch`, `empty`, bundle-selected item and special models);
- preservation of unknown JSON fields and unknown item model types;
- filesystem resource packs with pack stacking;
- a `ModAssetsResourcePack` source for Patchwork's
  `assets/<mod-name>/models/...` composed layout;
- small built-in definitions for `cube_all`, `cube_column`, `item/generated` and `item/handheld`;
- baked quads suitable for chunk meshing;
- grouping by texture and optional Bevy 0.19 mesh conversion;
- tests containing the eight JSON fixtures supplied with the project.

## Tip

Use `voxel-minecraft-models` by creating an `FsResourcePack` whose root contains `assets/`, parsing IDs with `ResourceLocation::parse`, resolving inheritance with `ModelResolver::new(&pack).resolve(&id)`, and converting the resulting `ResolvedModel` into `Vec<BakedQuad>` using `bake_model` or `bake_model_with_transform`; for blocks first call `pack.load_blockstate(&block_id)` and `select(&state_properties, deterministic_seed)`, then resolve each selected `model` and apply its `x`, `y`, and `uvlock` through `ModelTransform`; append visible quads to chunk geometry using `cull_face`, group by `texture` with `group_quads_by_texture`, or enable feature `bevy` and call `bevy::quads_to_bevy_meshes`; parse standalone legacy models/blockstates/modern item definitions with `parse_model`, `parse_blockstate`, `parse_item_definition`, or auto-detect using `parse_document`; runtime item-property evaluation, biome tints, special/entity models, transparency policy, texture loading/atlasing, and generated-item pixel-edge extrusion remain responsibilities of the host renderer.


## Parse one JSON document

```rust
use voxel_models_lib::{parse_document, JsonDocument};

let bytes = std::fs::read("assets/minecraft/models/block/stone.json")?;
match parse_document(&bytes)? {
    JsonDocument::Model(model) => println!("textures: {:?}", model.textures),
    JsonDocument::BlockState(state) => println!("variants: {}", state.variants.len()),
    JsonDocument::ItemDefinition(item) => println!("item tree: {:?}", item.model.kind),
}
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Resolve and bake a model

The root passed to `FsResourcePack` is the directory containing `assets/`.

```rust
use voxel_models_lib::{
    bake_model, BakeOptions, FsResourcePack, ModelResolver, ResourceLocation,
};

let pack = FsResourcePack::new("my_resource_pack");
let id = ResourceLocation::parse("minecraft:block/bamboo_block")?;
let resolved = ModelResolver::new(&pack).resolve(&id)?;
let quads = bake_model(&resolved, &BakeOptions::default())?;

for quad in quads {
    println!("{}: {:?}", quad.texture, quad.positions);
}
# Ok::<(), Box<dyn std::error::Error>>(())
```

Coordinates are normalized from Minecraft's `0..16` model space to `0..1` by default. Set `BakeOptions::normalize_coordinates` to `false` to retain model units.

## Patchwork composed assets

Patchwork copies every mod's `assets/` tree below a directory named after that
mod. Use `ModAssetsResourcePack` when the runtime root already is the composed
`assets/` directory:

```rust
use voxel_models_lib::{
    bake_model, BakeOptions, ModAssetsResourcePack, ModelResolver,
    ResourceLocation,
};

let pack = ModAssetsResourcePack::new("assets");
let id = ResourceLocation::parse("block-marble:block/marble")?;
let model = ModelResolver::new(&pack).resolve(&id)?;
let quads = bake_model(&model, &BakeOptions::default())?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

This resolves the ID above as
`assets/block-marble/models/block/marble.json`. Texture resource locations can
be mapped in the same way to
`assets/<namespace>/textures/<path>.png`. Cross-mod parent references should be
backed by formal Patchwork dependencies so the template asset mod is always
present in the composition.

## Select a blockstate

```rust
use std::collections::HashMap;
use voxel_models_lib::{ModelSource, FsResourcePack, ResourceLocation};

let pack = FsResourcePack::new("my_resource_pack");
let id = ResourceLocation::parse("minecraft:oak_stairs")?;
let definition = pack.load_blockstate(&id)?.expect("blockstate exists");
let state = HashMap::from([
    ("facing".to_owned(), "east".to_owned()),
    ("half".to_owned(), "bottom".to_owned()),
    ("shape".to_owned(), "straight".to_owned()),
]);

for selected in definition.select(&state, 12345)? {
    println!("model: {}", selected.model.model);
}
# Ok::<(), Box<dyn std::error::Error>>(())
```

Apply `selected.model.x`, `selected.model.y` and `selected.model.uvlock` through `ModelTransform` when baking:

```rust
use voxel_models_lib::{bake_model_with_transform, BakeOptions, ModelTransform};

let transform = ModelTransform {
    x_degrees: selected.model.x,
    y_degrees: selected.model.y,
    uv_lock: selected.model.uvlock,
};
let quads = bake_model_with_transform(&resolved, &BakeOptions::default(), transform)?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Bevy 0.19

```rust
use voxel_models_lib::bevy::quads_to_bevy_meshes;

let texture_meshes = quads_to_bevy_meshes(&quads);
for (texture_id, mesh) in texture_meshes {
    // Store the mesh in Assets<Mesh>, then associate texture_id with your
    // material, texture atlas page or texture-array layer.
    println!("mesh for {texture_id}");
}
```

A single standard Bevy material normally uses one texture, so the adapter returns one mesh per resolved texture. A chunk renderer using a texture array may instead consume `BakedQuad` directly and attach a custom layer index.

## Resource-pack stacking

`FsResourcePack` checks roots in reverse insertion order, matching the usual “last/highest pack wins” behavior:

```rust
let mut pack = FsResourcePack::new("vanilla_pack");
pack.push_root("user_pack");
```

## Run tests

```bash
cargo test
cargo check --features bevy
```

The uploaded fixtures are in `tests/fixtures/` and are exercised by `tests/uploaded_models.rs`.

## Intentional boundaries

This crate is a model-data parser and quad baker, not a complete Minecraft client renderer. Generated item textures currently produce front/back planes rather than pixel-outline side extrusion. Built-in entity/special item renderers, biome tint calculation, animated texture metadata, fluid meshing, transparency sorting and evaluation of runtime item properties belong in the host game. Unknown fields/types are retained so applications can add those behaviors without losing source data. `uvlock` is implemented for common right-angle blockstate rotations; unusual arbitrary rotations should be validated against the exact Minecraft version targeted by the game.
