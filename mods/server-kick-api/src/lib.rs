use bevy::prelude::*;
use player_network_message_types::PlayerId;
use std::net::SocketAddr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerKickTarget {
    Player(PlayerId),
    Address(SocketAddr),
}

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct ServerKickRequested {
    pub target: ServerKickTarget,
    pub reason: String,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServerKickSet {
    Apply,
}

pub trait ServerKickApi: Send + Sync + 'static {}
