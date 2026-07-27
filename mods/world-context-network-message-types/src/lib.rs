use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerWorldChanged {
    pub world_id: String,
    pub position: [f32; 3],
}
