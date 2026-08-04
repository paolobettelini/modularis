use bevy::prelude::*;
use bevy_mod::BevyMod;
use std::net::SocketAddr;
use tokio::task::JoinHandle;

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClientTransportConnected {
    pub server: SocketAddr,
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClientTransportDisconnected {
    pub server: SocketAddr,
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClientTransportDisconnectRequested;

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServerTransportConnected {
    pub address: SocketAddr,
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServerTransportDisconnected {
    pub address: SocketAddr,
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServerTransportDisconnectRequested {
    pub address: SocketAddr,
}

pub struct NetworkTransportEventsMod;

impl NetworkTransportEventsMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app
            .add_message::<ClientTransportConnected>()
            .add_message::<ClientTransportDisconnected>()
            .add_message::<ClientTransportDisconnectRequested>()
            .add_message::<ServerTransportConnected>()
            .add_message::<ServerTransportDisconnected>()
            .add_message::<ServerTransportDisconnectRequested>();
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
