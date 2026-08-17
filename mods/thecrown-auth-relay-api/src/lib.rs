use bevy::prelude::*;
use player_network_message_types::PlayerId;
use thecrown_protocol::{AccomodatePlayerData, PlayerIdentity};

#[derive(Message, Debug, Clone)]
pub struct RequestTheCrownAdmission {
    pub player_id: PlayerId,
    pub player: PlayerIdentity,
}

#[derive(Message, Debug, Clone)]
pub struct TheCrownAdmissionFinished {
    pub player_id: PlayerId,
    pub result: Result<AccomodatePlayerData, String>,
}

pub trait TheCrownAuthRelayApi: Send + Sync + 'static {}

