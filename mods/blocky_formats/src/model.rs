use crate::{BlockyError, Result, Quatf, Vec2f, Vec3f};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::Path};

/// Parsed `.blockymodel` file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockyModel {
    #[serde(default)]
    pub nodes: Vec<BlockyNode>,

    /// Optional format string, if present in the exporter version you use.
    #[serde(default)]
    pub format: Option<String>,

    /// Currently observed as `"auto"` in the plugin types.
    #[serde(default)]
    pub lod: Option<String>,

    /// Preserve unknown top-level fields for forward compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

impl BlockyModel {
    pub fn from_str(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json)?)
    }

    pub fn from_slice(json: &[u8]) -> Result<Self> {
        Ok(serde_json::from_slice(json)?)
    }

    pub fn from_reader<R: std::io::Read>(reader: R) -> Result<Self> {
        Ok(serde_json::from_reader(reader)?)
    }

    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let file = std::fs::File::open(path).map_err(|source| BlockyError::Io {
            path: path.to_path_buf(),
            source,
        })?;
        Self::from_reader(file)
    }
}

/// A node in `.blockymodel`. Nodes may contain a shape and child nodes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockyNode {
    pub id: String,
    pub name: String,

    #[serde(default)]
    pub position: Vec3f,

    #[serde(default)]
    pub orientation: Quatf,

    #[serde(default)]
    pub shape: Option<BlockyShape>,

    #[serde(default)]
    pub children: Vec<BlockyNode>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockyShape {
    #[serde(default)]
    pub offset: Vec3f,

    #[serde(default = "Vec3f::one_for_serde")]
    pub stretch: Vec3f,

    #[serde(default)]
    pub texture_layout: BTreeMap<String, UvFace>,

    #[serde(rename = "type", default)]
    pub shape_type: ShapeType,

    #[serde(default)]
    pub settings: ShapeSettings,

    #[serde(default)]
    pub unwrap_mode: Option<String>,

    #[serde(default = "default_true")]
    pub visible: bool,

    #[serde(default)]
    pub double_sided: bool,

    #[serde(default)]
    pub shading_mode: ShadingMode,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

impl Vec3f {
    pub(crate) fn one_for_serde() -> Self {
        Self::ONE
    }
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ShapeType {
    Box,
    Quad,
    #[default]
    None,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ShadingMode {
    Flat,
    #[default]
    Standard,
    Fullbright,
    Reflective,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuadNormal {
    #[serde(rename = "+X")]
    PosX,
    #[serde(rename = "+Y")]
    PosY,
    #[serde(rename = "+Z")]
    PosZ,
    #[serde(rename = "-X")]
    NegX,
    #[serde(rename = "-Y")]
    NegY,
    #[serde(rename = "-Z")]
    NegZ,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ShapeSettings {
    /// Dimensions of a shape. Box shapes normally use `{x, y, z}`.
    /// Quad shapes exported by the Hytale Blockbench plugin often use only `{x, y}`;
    /// in that case `Vec3f.z` defaults to `0.0`.
    #[serde(default)]
    pub size: Option<Vec3f>,

    /// Normal direction of a quad shape.
    #[serde(default)]
    pub normal: Option<QuadNormal>,

    #[serde(default)]
    pub is_piece: Option<bool>,

    #[serde(default)]
    pub is_static_box: Option<bool>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UvFace {
    #[serde(default)]
    pub offset: Vec2f,

    #[serde(default)]
    pub mirror: UvMirror,

    /// Expected values are usually 0, 90, 180, 270.
    #[serde(default)]
    pub angle: i32,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

impl Default for UvFace {
    fn default() -> Self {
        Self {
            offset: Vec2f::ZERO,
            mirror: UvMirror::default(),
            angle: 0,
            extra: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct UvMirror {
    #[serde(default)]
    pub x: bool,
    #[serde(default)]
    pub y: bool,
}
