use serde::{Deserialize, Serialize};
use thecrown_protocol::{PlayerStatus, ServerEntry};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NetworkOverview {
    pub online_players: u64,
    pub physical_servers: usize,
    pub active_instances: usize,
    pub hubs: Vec<ServerEntry>,
    pub parkour: Vec<ServerEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlayerProfile {
    pub uuid: String,
    pub username: String,
    pub parkour_record: i32,
    pub status: PlayerStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ViewerProfile {
    pub uuid: String,
    pub username: String,
    pub status: PlayerStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TransferRequestResult {
    pub accepted: bool,
    pub message: String,
}
