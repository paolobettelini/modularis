use player_network_message_types::{NetworkPlayer, PlayerId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JoinRequest {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LeaveRequest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinAccepted {
    pub player_id: PlayerId,
    pub players: Vec<NetworkPlayer>,
}
