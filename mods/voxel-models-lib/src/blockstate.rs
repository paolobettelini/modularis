use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use crate::{Error, ResourceLocation, Result};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockStateModel {
    pub model: ResourceLocation,
    #[serde(default)]
    pub x: i32,
    #[serde(default)]
    pub y: i32,
    #[serde(default)]
    pub uvlock: bool,
    #[serde(default = "default_weight")]
    pub weight: u32,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

fn default_weight() -> u32 {
    1
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModelChoice {
    One(BlockStateModel),
    Many(Vec<BlockStateModel>),
}

impl ModelChoice {
    pub fn entries(&self) -> &[BlockStateModel] {
        match self {
            Self::One(value) => std::slice::from_ref(value),
            Self::Many(values) => values,
        }
    }

    pub fn choose(&self, seed: u64) -> Option<&BlockStateModel> {
        let entries = self.entries();
        if entries.is_empty() {
            return None;
        }
        let total: u64 = entries.iter().map(|entry| entry.weight.max(1) as u64).sum();
        let mut cursor = seed % total;
        for entry in entries {
            let weight = entry.weight.max(1) as u64;
            if cursor < weight {
                return Some(entry);
            }
            cursor -= weight;
        }
        entries.last()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyMatcher {
    AnyOf(Vec<String>),
}

impl PropertyMatcher {
    fn matches(&self, value: Option<&str>) -> bool {
        match self {
            Self::AnyOf(values) => value
                .map(|actual| values.iter().any(|expected| expected == actual))
                .unwrap_or(false),
        }
    }
}

impl<'de> Deserialize<'de> for PropertyMatcher {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        let values = match value {
            Value::String(value) => value.split('|').map(str::to_owned).collect(),
            Value::Array(values) => values
                .into_iter()
                .map(|value| match value {
                    Value::String(value) => Ok(value),
                    other => Err(serde::de::Error::custom(format!(
                        "property matcher array contains {other}"
                    ))),
                })
                .collect::<std::result::Result<Vec<_>, D::Error>>()?,
            other => {
                return Err(serde::de::Error::custom(format!(
                    "property matcher must be string or array, got {other}"
                )));
            }
        };
        Ok(Self::AnyOf(values))
    }
}

impl Serialize for PropertyMatcher {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::AnyOf(values) => values.join("|").serialize(serializer),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WhenClause {
    Properties(BTreeMap<String, PropertyMatcher>),
    Or(Vec<WhenClause>),
    And(Vec<WhenClause>),
}

impl WhenClause {
    pub fn matches(&self, state: &HashMap<String, String>) -> bool {
        match self {
            Self::Properties(properties) => properties
                .iter()
                .all(|(name, matcher)| matcher.matches(state.get(name).map(String::as_str))),
            Self::Or(clauses) => clauses.iter().any(|clause| clause.matches(state)),
            Self::And(clauses) => clauses.iter().all(|clause| clause.matches(state)),
        }
    }
}

impl<'de> Deserialize<'de> for WhenClause {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut object = BTreeMap::<String, Value>::deserialize(deserializer)?;
        if let Some(value) = object.remove("OR") {
            let clauses = serde_json::from_value(value).map_err(serde::de::Error::custom)?;
            return Ok(Self::Or(clauses));
        }
        if let Some(value) = object.remove("AND") {
            let clauses = serde_json::from_value(value).map_err(serde::de::Error::custom)?;
            return Ok(Self::And(clauses));
        }
        let properties = object
            .into_iter()
            .map(|(key, value)| {
                serde_json::from_value(value)
                    .map(|matcher| (key, matcher))
                    .map_err(serde::de::Error::custom)
            })
            .collect::<std::result::Result<BTreeMap<_, _>, D::Error>>()?;
        Ok(Self::Properties(properties))
    }
}

impl Serialize for WhenClause {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Properties(properties) => properties.serialize(serializer),
            Self::Or(clauses) => {
                let mut object = BTreeMap::new();
                object.insert("OR", clauses);
                object.serialize(serializer)
            }
            Self::And(clauses) => {
                let mut object = BTreeMap::new();
                object.insert("AND", clauses);
                object.serialize(serializer)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultipartCase {
    #[serde(default)]
    pub when: Option<WhenClause>,
    pub apply: ModelChoice,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BlockStateDefinition {
    #[serde(default)]
    pub variants: BTreeMap<String, ModelChoice>,
    #[serde(default)]
    pub multipart: Vec<MultipartCase>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Copy)]
pub struct BlockStateSelection<'a> {
    pub model: &'a BlockStateModel,
    pub multipart_index: Option<usize>,
}

impl BlockStateDefinition {
    pub fn select<'a>(
        &'a self,
        state: &HashMap<String, String>,
        seed: u64,
    ) -> Result<Vec<BlockStateSelection<'a>>> {
        let mut selected = Vec::new();

        if !self.variants.is_empty() {
            let mut best: Option<(&ModelChoice, usize)> = None;
            for (key, choice) in &self.variants {
                let predicates = parse_variant_key(key)?;
                if predicates
                    .iter()
                    .all(|(name, expected)| state.get(name) == Some(expected))
                {
                    let specificity = predicates.len();
                    if best.map(|(_, count)| specificity > count).unwrap_or(true) {
                        best = Some((choice, specificity));
                    }
                }
            }
            if let Some((choice, _)) = best {
                if let Some(model) = choice.choose(mix_seed(seed, 0)) {
                    selected.push(BlockStateSelection {
                        model,
                        multipart_index: None,
                    });
                }
            }
        }

        for (index, case) in self.multipart.iter().enumerate() {
            if case
                .when
                .as_ref()
                .map(|when| when.matches(state))
                .unwrap_or(true)
            {
                if let Some(model) = case.apply.choose(mix_seed(seed, index as u64 + 1)) {
                    selected.push(BlockStateSelection {
                        model,
                        multipart_index: Some(index),
                    });
                }
            }
        }

        Ok(selected)
    }
}

fn parse_variant_key(key: &str) -> Result<Vec<(String, String)>> {
    if key.is_empty() {
        return Ok(Vec::new());
    }
    key.split(',')
        .map(|entry| {
            let (name, value) = entry
                .split_once('=')
                .ok_or_else(|| Error::InvalidVariantKey(key.to_owned()))?;
            if name.is_empty() || value.is_empty() {
                return Err(Error::InvalidVariantKey(key.to_owned()));
            }
            Ok((name.to_owned(), value.to_owned()))
        })
        .collect()
}

fn mix_seed(seed: u64, salt: u64) -> u64 {
    let mut value = seed ^ salt.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    value ^= value >> 30;
    value = value.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^ (value >> 31)
}
