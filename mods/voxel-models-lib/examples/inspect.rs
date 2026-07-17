use std::{env, fs};

use voxel_models_lib::{JsonDocument, parse_document};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args()
        .nth(1)
        .ok_or("usage: cargo run --example inspect -- file.json")?;
    let bytes = fs::read(path)?;
    match parse_document(&bytes)? {
        JsonDocument::Model(model) => println!(
            "model: parent={:?}, textures={}, elements={}",
            model.parent,
            model.textures.len(),
            model.elements.as_ref().map(Vec::len).unwrap_or(0)
        ),
        JsonDocument::BlockState(state) => println!(
            "blockstate: variants={}, multipart={}",
            state.variants.len(),
            state.multipart.len()
        ),
        JsonDocument::ItemDefinition(item) => println!("item definition: {:?}", item.model.kind),
    }
    Ok(())
}
