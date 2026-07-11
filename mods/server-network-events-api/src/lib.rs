use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::ClientBoundMessage;
use player_network_message_types::PlayerId;
use std::net::SocketAddr;
use tokio::task::JoinHandle;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerAudience {
    Address(SocketAddr),
    Player(PlayerId),
    Broadcast,
    BroadcastExceptAddress(SocketAddr),
    BroadcastExceptPlayer(PlayerId),
    Players(Vec<PlayerId>),
}

#[derive(Message, Debug, Clone)]
pub struct ServerPacketOut {
    pub audience: ServerAudience,
    pub message: ClientBoundMessage,
}

pub struct ServerNetworkEventsApiMod;

impl ServerNetworkEventsApiMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app.add_message::<ServerPacketOut>();
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

pub trait ServerNetworkEventsApi: Send + Sync + 'static {}

impl ServerNetworkEventsApi for ServerNetworkEventsApiMod {}
