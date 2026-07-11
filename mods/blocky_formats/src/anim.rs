use crate::{BlockyError, Quatf, Result, Vec2f, Vec3f};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::Path};

/// Blocky animation files express time in frames at 60 FPS in the official plugin.
pub const BLOCKYANIM_FPS: f32 = 60.0;

/// Parsed `.blockyanim` file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockyAnimation {
    pub format_version: u32,

    /// Duration in frames, not seconds.
    pub duration: f32,

    #[serde(default)]
    pub hold_last_keyframe: bool,

    #[serde(default)]
    pub node_animations: BTreeMap<String, NodeAnimation>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

impl BlockyAnimation {
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

    pub fn duration_seconds(&self) -> f32 {
        self.duration / BLOCKYANIM_FPS
    }

    /// Converts seconds to the frame units used by `.blockyanim`.
    pub fn seconds_to_frames(seconds: f32) -> f32 {
        seconds * BLOCKYANIM_FPS
    }

    /// Normalizes a sample time according to `holdLastKeyframe`.
    pub fn normalize_time_seconds(&self, seconds: f32) -> f32 {
        let duration_seconds = self.duration_seconds();
        if duration_seconds <= 0.0 {
            return 0.0;
        }

        let t = seconds.max(0.0);
        if self.hold_last_keyframe {
            t.min(duration_seconds)
        } else {
            t % duration_seconds
        }
    }

    pub fn node_animation(&self, node_name: &str) -> Option<&NodeAnimation> {
        self.node_animations.get(node_name)
    }
}

/// Animation tracks for one node, keyed by node name in the file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeAnimation {
    #[serde(default)]
    pub position: Vec<Keyframe<Vec3f>>,

    #[serde(default)]
    pub orientation: Vec<Keyframe<Quatf>>,

    #[serde(default)]
    pub shape_stretch: Vec<Keyframe<Vec3f>>,

    #[serde(default)]
    pub shape_visible: Vec<Keyframe<bool>>,

    #[serde(default)]
    pub shape_uv_offset: Vec<Keyframe<Vec2f>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

impl NodeAnimation {
    pub fn is_empty(&self) -> bool {
        self.position.is_empty()
            && self.orientation.is_empty()
            && self.shape_stretch.is_empty()
            && self.shape_visible.is_empty()
            && self.shape_uv_offset.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Keyframe<T> {
    /// Time in frames, not seconds.
    pub time: f32,

    pub delta: T,

    #[serde(default)]
    pub interpolation_type: Option<InterpolationType>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

impl<T> Keyframe<T> {
    pub fn time_seconds(&self) -> f32 {
        self.time / BLOCKYANIM_FPS
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum InterpolationType {
    Smooth,
    #[default]
    Linear,
    #[serde(other)]
    Unknown,
}
