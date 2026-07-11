use bevy::prelude::*;
use player_network_message_types::PlayerId;

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServerPlayerJoined {
    pub player_id: PlayerId,
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServerPlayerLeft {
    pub player_id: PlayerId,
}
