use player_network_message_types::PlayerId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerGravityChanged {
    pub player_id: PlayerId,
    pub gravity: [f32; 3],
}
