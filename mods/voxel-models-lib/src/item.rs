use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use crate::ResourceLocation;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemDefinition {
    pub model: ItemModel,
    #[serde(default)]
    pub hand_animation_on_swap: Option<bool>,
    #[serde(default)]
    pub oversize_in_gui: Option<bool>,
    #[serde(default)]
    pub swap_animation_scale: Option<f32>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemProperty(pub String);

impl Serialize for ItemProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for ItemProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self(String::deserialize(deserializer)?))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ItemModel {
    pub kind: ItemModelKind,
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ItemModelKind {
    Model {
        model: ResourceLocation,
        tints: Vec<Value>,
    },
    Composite {
        models: Vec<ItemModel>,
    },
    Condition {
        property: ItemProperty,
        on_true: Box<ItemModel>,
        on_false: Box<ItemModel>,
    },
    Select {
        property: ItemProperty,
        cases: Vec<ItemModelCase>,
        fallback: Option<Box<ItemModel>>,
    },
    RangeDispatch {
        property: ItemProperty,
        scale: f32,
        entries: Vec<ItemModelEntry>,
        fallback: Option<Box<ItemModel>>,
    },
    Empty,
    BundleSelectedItem,
    Special {
        base: ResourceLocation,
        model: Value,
    },
    Unknown {
        type_name: String,
        value: Value,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemModelCase {
    #[serde(alias = "when")]
    pub values: Value,
    pub model: ItemModel,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemModelEntry {
    pub threshold: f32,
    pub model: ItemModel,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

impl<'de> Deserialize<'de> for ItemModel {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        parse_item_model(value).map_err(serde::de::Error::custom)
    }
}

impl Serialize for ItemModel {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        item_model_to_value(self)
            .map_err(serde::ser::Error::custom)?
            .serialize(serializer)
    }
}

fn parse_item_model(value: Value) -> Result<ItemModel, String> {
    let mut object = value
        .as_object()
        .cloned()
        .ok_or_else(|| "item model must be an object".to_owned())?;
    let type_name = object
        .remove("type")
        .and_then(|value| value.as_str().map(str::to_owned))
        .ok_or_else(|| "item model is missing string field `type`".to_owned())?;

    let kind = match type_name.as_str() {
        "minecraft:model" | "model" => {
            let model = take_location(&mut object, "model")?;
            let tints = object
                .remove("tints")
                .map(|value| serde_json::from_value(value).map_err(|error| error.to_string()))
                .transpose()?
                .unwrap_or_default();
            ItemModelKind::Model { model, tints }
        }
        "minecraft:composite" | "composite" => {
            let models = take(&mut object, "models")?;
            ItemModelKind::Composite { models }
        }
        "minecraft:condition" | "condition" => {
            let property = take(&mut object, "property")?;
            let on_true = Box::new(take(&mut object, "on_true")?);
            let on_false = Box::new(take(&mut object, "on_false")?);
            ItemModelKind::Condition {
                property,
                on_true,
                on_false,
            }
        }
        "minecraft:select" | "select" => {
            let property = take(&mut object, "property")?;
            let cases = take(&mut object, "cases")?;
            let fallback = object
                .remove("fallback")
                .map(|value| serde_json::from_value(value).map(Box::new))
                .transpose()
                .map_err(|error| error.to_string())?;
            ItemModelKind::Select {
                property,
                cases,
                fallback,
            }
        }
        "minecraft:range_dispatch" | "range_dispatch" => {
            let property = take(&mut object, "property")?;
            let scale = object
                .remove("scale")
                .map(|value| serde_json::from_value(value).map_err(|error| error.to_string()))
                .transpose()?
                .unwrap_or(1.0);
            let entries = take(&mut object, "entries")?;
            let fallback = object
                .remove("fallback")
                .map(|value| serde_json::from_value(value).map(Box::new))
                .transpose()
                .map_err(|error| error.to_string())?;
            ItemModelKind::RangeDispatch {
                property,
                scale,
                entries,
                fallback,
            }
        }
        "minecraft:empty" | "empty" => ItemModelKind::Empty,
        "minecraft:bundle/selected_item" | "bundle/selected_item" => {
            ItemModelKind::BundleSelectedItem
        }
        "minecraft:special" | "special" => {
            let base = take_location(&mut object, "base")?;
            let model = object.remove("model").unwrap_or(Value::Null);
            ItemModelKind::Special { base, model }
        }
        _ => ItemModelKind::Unknown {
            type_name: type_name.clone(),
            value: Value::Object(object.clone()),
        },
    };

    Ok(ItemModel {
        kind,
        extra: object.into_iter().collect(),
    })
}

fn item_model_to_value(model: &ItemModel) -> Result<Value, serde_json::Error> {
    let mut object = serde_json::Map::new();
    match &model.kind {
        ItemModelKind::Model { model, tints } => {
            object.insert("type".into(), Value::String("minecraft:model".into()));
            object.insert("model".into(), Value::String(model.to_string()));
            if !tints.is_empty() {
                object.insert("tints".into(), serde_json::to_value(tints)?);
            }
        }
        ItemModelKind::Composite { models } => {
            object.insert("type".into(), Value::String("minecraft:composite".into()));
            object.insert("models".into(), serde_json::to_value(models)?);
        }
        ItemModelKind::Condition {
            property,
            on_true,
            on_false,
        } => {
            object.insert("type".into(), Value::String("minecraft:condition".into()));
            object.insert("property".into(), Value::String(property.0.clone()));
            object.insert("on_true".into(), serde_json::to_value(on_true)?);
            object.insert("on_false".into(), serde_json::to_value(on_false)?);
        }
        ItemModelKind::Select {
            property,
            cases,
            fallback,
        } => {
            object.insert("type".into(), Value::String("minecraft:select".into()));
            object.insert("property".into(), Value::String(property.0.clone()));
            object.insert("cases".into(), serde_json::to_value(cases)?);
            if let Some(fallback) = fallback {
                object.insert("fallback".into(), serde_json::to_value(fallback)?);
            }
        }
        ItemModelKind::RangeDispatch {
            property,
            scale,
            entries,
            fallback,
        } => {
            object.insert(
                "type".into(),
                Value::String("minecraft:range_dispatch".into()),
            );
            object.insert("property".into(), Value::String(property.0.clone()));
            object.insert("scale".into(), serde_json::to_value(scale)?);
            object.insert("entries".into(), serde_json::to_value(entries)?);
            if let Some(fallback) = fallback {
                object.insert("fallback".into(), serde_json::to_value(fallback)?);
            }
        }
        ItemModelKind::Empty => {
            object.insert("type".into(), Value::String("minecraft:empty".into()));
        }
        ItemModelKind::BundleSelectedItem => {
            object.insert(
                "type".into(),
                Value::String("minecraft:bundle/selected_item".into()),
            );
        }
        ItemModelKind::Special { base, model } => {
            object.insert("type".into(), Value::String("minecraft:special".into()));
            object.insert("base".into(), Value::String(base.to_string()));
            object.insert("model".into(), model.clone());
        }
        ItemModelKind::Unknown { type_name, value } => {
            object.insert("type".into(), Value::String(type_name.clone()));
            if let Value::Object(values) = value {
                object.extend(values.clone());
            }
        }
    }
    object.extend(
        model
            .extra
            .iter()
            .map(|(key, value)| (key.clone(), value.clone())),
    );
    Ok(Value::Object(object))
}

fn take<T: serde::de::DeserializeOwned>(
    object: &mut serde_json::Map<String, Value>,
    key: &str,
) -> Result<T, String> {
    let value = object
        .remove(key)
        .ok_or_else(|| format!("item model is missing `{key}`"))?;
    serde_json::from_value(value).map_err(|error| error.to_string())
}

fn take_location(
    object: &mut serde_json::Map<String, Value>,
    key: &str,
) -> Result<ResourceLocation, String> {
    take(object, key)
}
