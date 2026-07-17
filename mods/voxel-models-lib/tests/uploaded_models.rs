use std::{fs, path::PathBuf};

use voxel_models_lib::{JsonDocument, ResourceLocation, parse_document};

fn fixture(name: &str) -> Vec<u8> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name);
    fs::read(path).expect("fixture must be readable")
}

#[test]
fn all_uploaded_files_parse_as_models() {
    for name in [
        "bamboo_block.json",
        "nether_wart_block.json",
        "acacia_chest_boat.json",
        "air.json",
        "bogged_spawn_egg.json",
        "candle.json",
        "cherry_door.json",
        "compass_07.json",
    ] {
        let document = parse_document(&fixture(name)).expect(name);
        assert!(matches!(document, JsonDocument::Model(_)), "{name}");
    }
}

#[test]
fn bamboo_block_keeps_parent_and_texture_variables() {
    let JsonDocument::Model(model) = parse_document(&fixture("bamboo_block.json")).unwrap() else {
        panic!("expected model")
    };
    assert_eq!(
        model.parent,
        Some(ResourceLocation::parse("minecraft:block/cube_column").unwrap())
    );
    assert_eq!(model.textures["end"], "minecraft:block/bamboo_block_top");
    assert_eq!(model.textures["side"], "minecraft:block/bamboo_block");
    assert!(model.elements.is_none());
}

#[test]
fn generated_item_models_keep_layer_zero() {
    for (name, texture) in [
        ("acacia_chest_boat.json", "minecraft:item/acacia_chest_boat"),
        ("bogged_spawn_egg.json", "minecraft:item/bogged_spawn_egg"),
        ("candle.json", "minecraft:item/candle"),
        ("cherry_door.json", "minecraft:item/cherry_door"),
        ("compass_07.json", "minecraft:item/compass_07"),
    ] {
        let JsonDocument::Model(model) = parse_document(&fixture(name)).unwrap() else {
            panic!("expected model")
        };
        assert_eq!(model.textures["layer0"], texture);
        assert_eq!(
            model.parent,
            Some(ResourceLocation::parse("minecraft:item/generated").unwrap())
        );
    }
}

#[test]
fn air_model_without_parent_is_valid() {
    let JsonDocument::Model(model) = parse_document(&fixture("air.json")).unwrap() else {
        panic!("expected model")
    };
    assert!(model.parent.is_none());
    assert_eq!(model.textures["particle"], "minecraft:missingno");
    assert!(model.elements.is_none());
}
