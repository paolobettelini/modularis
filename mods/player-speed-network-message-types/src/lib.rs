use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerSpeedChanged {
    pub multiplier: f32,
}
