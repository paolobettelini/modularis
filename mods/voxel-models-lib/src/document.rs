use serde_json::Value;

use crate::{BlockStateDefinition, Error, ItemDefinition, Model, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum JsonDocument {
    Model(Model),
    BlockState(BlockStateDefinition),
    ItemDefinition(ItemDefinition),
}

pub fn parse_document(bytes: &[u8]) -> Result<JsonDocument> {
    let value: Value = serde_json::from_slice(bytes)?;
    let object = value
        .as_object()
        .ok_or_else(|| Error::UnsupportedDocument("top level must be an object".into()))?;

    if object.contains_key("variants") || object.contains_key("multipart") {
        return Ok(JsonDocument::BlockState(serde_json::from_value(value)?));
    }

    if object
        .get("model")
        .and_then(Value::as_object)
        .and_then(|model| model.get("type"))
        .is_some()
    {
        return Ok(JsonDocument::ItemDefinition(serde_json::from_value(value)?));
    }

    Ok(JsonDocument::Model(serde_json::from_value(value)?))
}
