use std::collections::HashMap;

use voxel_models_lib::{
    ItemModelKind, JsonDocument, parse_blockstate, parse_document, parse_item_definition,
};

#[test]
fn detects_blockstates() {
    let json = br#"{
        "variants": {
            "": { "model": "example:block/base" },
            "facing=north,powered=true": { "model": "example:block/on", "y": 180 }
        },
        "multipart": [
            {
                "when": { "OR": [{"north": "true"}, {"south": "true|connected"}] },
                "apply": { "model": "example:block/arm" }
            }
        ]
    }"#;
    assert!(matches!(
        parse_document(json).unwrap(),
        JsonDocument::BlockState(_)
    ));

    let blockstate = parse_blockstate(json).unwrap();
    let state = HashMap::from([
        ("facing".to_owned(), "north".to_owned()),
        ("powered".to_owned(), "true".to_owned()),
        ("south".to_owned(), "connected".to_owned()),
    ]);
    let selected = blockstate.select(&state, 42).unwrap();
    assert_eq!(selected.len(), 2);
    assert_eq!(selected[0].model.model.to_string(), "example:block/on");
    assert_eq!(selected[1].model.model.to_string(), "example:block/arm");
}

#[test]
fn parses_modern_item_definition_tree() {
    let json = br#"{
        "model": {
            "type": "minecraft:condition",
            "property": "minecraft:using_item",
            "on_true": {
                "type": "minecraft:model",
                "model": "example:item/active"
            },
            "on_false": {
                "type": "minecraft:composite",
                "models": [
                    {"type": "minecraft:model", "model": "example:item/base"},
                    {"type": "minecraft:empty"}
                ]
            }
        }
    }"#;
    let definition = parse_item_definition(json).unwrap();
    let ItemModelKind::Condition { property, .. } = definition.model.kind else {
        panic!("condition expected")
    };
    assert_eq!(property.0, "minecraft:using_item");
}

#[test]
fn unknown_item_model_types_are_preserved() {
    let json = br#"{
        "model": {
            "type": "modded:custom_renderer",
            "answer": 42
        }
    }"#;
    let definition = parse_item_definition(json).unwrap();
    let ItemModelKind::Unknown { type_name, value } = definition.model.kind else {
        panic!("unknown kind expected")
    };
    assert_eq!(type_name, "modded:custom_renderer");
    assert_eq!(value["answer"], 42);
}
