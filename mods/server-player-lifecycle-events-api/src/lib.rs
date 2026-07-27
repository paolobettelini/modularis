use bevy::prelude::*;
use player_network_message_types::PlayerId;

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServerPlayerJoined {
    pub player_id: PlayerId,
}

/// Emitted after `JoinAccepted` has been queued.
///
/// Initialization policies should use `ServerPlayerJoined`; systems that must
/// send follow-up packets after the acceptance packet should use this message.
#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServerPlayerReady {
    pub player_id: PlayerId,
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServerPlayerLeft {
    pub player_id: PlayerId,
}
