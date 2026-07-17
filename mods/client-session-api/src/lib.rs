use bevy::prelude::*;
use player_network_message_types::PlayerId;

#[derive(Resource, Debug, Default)]
pub struct ClientSession {
    pub player_id: Option<PlayerId>,
    pub disconnect_reason: Option<String>,
}

pub trait ClientSessionApi: Send + Sync + 'static {}
