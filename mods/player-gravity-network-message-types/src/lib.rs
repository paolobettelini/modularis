use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerGravityChanged {
    pub gravity: [f32; 3],
}
