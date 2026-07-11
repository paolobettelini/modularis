use serde::{Deserialize, Serialize};

pub type PlayerId = u64;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NetworkPlayer {
    pub id: PlayerId,
    pub name: String,
    pub position: [f32; 3],
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerMove {
    pub position: [f32; 3],
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerJoined {
    pub player: NetworkPlayer,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlayerLeft {
    pub player_id: PlayerId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerMoved {
    pub player_id: PlayerId,
    pub position: [f32; 3],
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerRotationChanged {
    pub player_id: PlayerId,
    pub yaw: f32,
    pub pitch: f32,
}
