use std::{fs, path::Path};

use tempfile::tempdir;
use voxel_models_lib::{
    BakeOptions, FsResourcePack, ModAssetsResourcePack, ModelResolver, ResourceLocation,
    bake_model, bake_model_boxes, group_quads_by_texture,
};

fn put(root: &Path, relative: &str, contents: &str) {
    let path = root.join(relative);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, contents).unwrap();
}

#[test]
fn resolves_cross_mod_parent_in_patchwork_asset_layout() {
    let dir = tempdir().unwrap();
    put(
        dir.path(),
        "template-mod/models/block/cube.json",
        r##"{
            "parent": "minecraft:block/cube_all",
            "textures": { "all": "#surface" }
        }"##,
    );
    put(
        dir.path(),
        "block-example/models/block/example.json",
        r##"{
            "parent": "template-mod:block/cube",
            "textures": { "surface": "block-example:block/example" }
        }"##,
    );

    let pack = ModAssetsResourcePack::new(dir.path());
    let id = ResourceLocation::parse("block-example:block/example").unwrap();
    let resolved = ModelResolver::new(&pack).resolve(&id).unwrap();
    let quads = bake_model(&resolved, &BakeOptions::default()).unwrap();

    assert_eq!(quads.len(), 6);
    assert!(
        quads
            .iter()
            .all(|quad| quad.texture.to_string() == "block-example:block/example")
    );
}

#[test]
fn resolves_builtin_cube_parent_and_bakes_six_quads() {
    let dir = tempdir().unwrap();
    put(
        dir.path(),
        "assets/example/models/block/test.json",
        r##"{
            "parent": "minecraft:block/cube_all",
            "textures": { "all": "example:block/test" }
        }"##,
    );
    let pack = FsResourcePack::new(dir.path());
    let id = ResourceLocation::parse("example:block/test").unwrap();
    let resolved = ModelResolver::new(&pack).resolve(&id).unwrap();
    assert_eq!(resolved.elements.len(), 1);
    assert_eq!(resolved.lineage.len(), 2);

    let quads = bake_model(&resolved, &BakeOptions::default()).unwrap();
    assert_eq!(quads.len(), 6);
    assert!(
        quads
            .iter()
            .all(|quad| quad.texture.to_string() == "example:block/test")
    );
    assert_eq!(group_quads_by_texture(&quads).len(), 1);
}

#[test]
fn generated_item_parent_bakes_front_and_back() {
    let dir = tempdir().unwrap();
    put(
        dir.path(),
        "assets/example/models/item/token.json",
        r##"{
            "parent": "minecraft:item/generated",
            "textures": { "layer0": "example:item/token" }
        }"##,
    );
    let pack = FsResourcePack::new(dir.path());
    let id = ResourceLocation::parse("example:item/token").unwrap();
    let resolved = ModelResolver::new(&pack).resolve(&id).unwrap();
    assert!(resolved.generated_item);
    let quads = bake_model(&resolved, &BakeOptions::default()).unwrap();
    assert_eq!(quads.len(), 2);
}

#[test]
fn child_textures_override_parent_variables() {
    let dir = tempdir().unwrap();
    put(
        dir.path(),
        "assets/example/models/block/parent.json",
        r##"{
            "parent": "minecraft:block/cube_all",
            "textures": { "all": "example:block/parent" }
        }"##,
    );
    put(
        dir.path(),
        "assets/example/models/block/child.json",
        r##"{
            "parent": "example:block/parent",
            "textures": { "all": "example:block/child" }
        }"##,
    );
    let pack = FsResourcePack::new(dir.path());
    let id = ResourceLocation::parse("example:block/child").unwrap();
    let resolved = ModelResolver::new(&pack).resolve(&id).unwrap();
    let quads = bake_model(&resolved, &BakeOptions::default()).unwrap();
    assert!(
        quads
            .iter()
            .all(|quad| quad.texture.to_string() == "example:block/child")
    );
}

#[test]
fn model_elements_become_independent_normalized_boxes() {
    let dir = tempdir().unwrap();
    put(
        dir.path(),
        "assets/example/models/block/steps.json",
        r##"{
            "textures": { "all": "example:block/steps" },
            "elements": [
                { "from": [0, 0, 0], "to": [16, 8, 16], "faces": {} },
                { "from": [8, 8, 0], "to": [16, 16, 16], "faces": {} }
            ]
        }"##,
    );
    let pack = FsResourcePack::new(dir.path());
    let id = ResourceLocation::parse("example:block/steps").unwrap();
    let resolved = ModelResolver::new(&pack).resolve(&id).unwrap();
    let boxes = bake_model_boxes(&resolved, &BakeOptions::default());

    assert_eq!(boxes.len(), 2);
    assert_eq!(boxes[0].min, [0.0, 0.0, 0.0]);
    assert_eq!(boxes[0].max, [1.0, 0.5, 1.0]);
    assert_eq!(boxes[1].min, [0.5, 0.5, 0.0]);
    assert_eq!(boxes[1].max, [1.0, 1.0, 1.0]);
}
