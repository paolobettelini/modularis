use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkyColorChanged {
    pub color: [f32; 4],
}
