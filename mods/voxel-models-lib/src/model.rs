use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ResourceLocation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Down,
    Up,
    North,
    South,
    West,
    East,
}

impl Direction {
    pub const ALL: [Self; 6] = [
        Self::Down,
        Self::Up,
        Self::North,
        Self::South,
        Self::West,
        Self::East,
    ];

    pub fn normal(self) -> [f32; 3] {
        match self {
            Self::Down => [0.0, -1.0, 0.0],
            Self::Up => [0.0, 1.0, 0.0],
            Self::North => [0.0, 0.0, -1.0],
            Self::South => [0.0, 0.0, 1.0],
            Self::West => [-1.0, 0.0, 0.0],
            Self::East => [1.0, 0.0, 0.0],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GuiLight {
    Front,
    Side,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisplayContext {
    ThirdpersonRighthand,
    ThirdpersonLefthand,
    FirstpersonRighthand,
    FirstpersonLefthand,
    Gui,
    Head,
    Ground,
    Fixed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DisplayTransform {
    #[serde(default)]
    pub rotation: Option<[f32; 3]>,
    #[serde(default)]
    pub translation: Option<[f32; 3]>,
    #[serde(default)]
    pub scale: Option<[f32; 3]>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElementRotation {
    pub origin: [f32; 3],
    pub axis: Axis,
    pub angle: f32,
    #[serde(default)]
    pub rescale: bool,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelFace {
    #[serde(default)]
    pub uv: Option<[f32; 4]>,
    pub texture: String,
    #[serde(default)]
    pub cullface: Option<Direction>,
    #[serde(default)]
    pub rotation: u16,
    #[serde(default)]
    pub tintindex: Option<i32>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Element {
    pub from: [f32; 3],
    pub to: [f32; 3],
    #[serde(default)]
    pub rotation: Option<ElementRotation>,
    #[serde(default = "default_true")]
    pub shade: bool,
    #[serde(default)]
    pub light_emission: Option<u8>,
    pub faces: BTreeMap<Direction, ModelFace>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

fn default_true() -> bool {
    true
}

/// Raw model JSON. Optional fields are retained as `Option` so inheritance can
/// distinguish “not present” from an explicit empty value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Model {
    #[serde(default)]
    pub parent: Option<ResourceLocation>,
    #[serde(default)]
    pub ambientocclusion: Option<bool>,
    #[serde(default)]
    pub gui_light: Option<GuiLight>,
    #[serde(default)]
    pub textures: BTreeMap<String, String>,
    #[serde(default)]
    pub elements: Option<Vec<Element>>,
    #[serde(default)]
    pub display: BTreeMap<DisplayContext, DisplayTransform>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}
