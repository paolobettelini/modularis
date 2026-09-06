use serde::{Deserialize, Serialize};

pub type PlayerId = u64;
pub type MovementEpoch = u64;
pub type MovementSequence = u64;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NetworkPlayer {
    pub id: PlayerId,
    pub name: String,
    pub position: [f32; 3],
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Debug,Clone,Copy,Serialize,Deserialize,PartialEq)]
pub struct PlayerSurfacePosition {pub surface:u128,pub local_foot:[f32;3]}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerMove {
    /// Untrusted support-relative position; validated against server contact history.
    pub surface:Option<PlayerSurfacePosition>,
    pub movement_epoch: MovementEpoch,
    pub sequence: MovementSequence,
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
    pub movement_epoch: MovementEpoch,
    pub acknowledged_sequence: Option<MovementSequence>,
    /// Whether the local predicted player must reconcile to `position`.
    /// Remote players always use the position regardless of this flag.
    pub correction: bool,
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
