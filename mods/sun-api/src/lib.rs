use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SunSettings {
    /// Position of the sun relative to the world origin. A directional renderer
    /// uses its normalized inverse as the light direction.
    pub position: [f32; 3],
    pub illuminance: f32,
    pub color: [f32; 3],
}

impl Default for SunSettings {
    fn default() -> Self {
        Self {
            position: [0.45, 0.82, 0.35],
            illuminance: 12_000.0,
            color: [1.0, 0.94, 0.82],
        }
    }
}
