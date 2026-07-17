use player_network_message_types::PlayerId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerScaleChanged {
    pub player_id: PlayerId,
    pub scale: f32,
}
