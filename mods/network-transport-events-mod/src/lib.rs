use bevy::prelude::*;
use bevy_mod::BevyMod;
use std::net::SocketAddr;
use std::sync::{Arc, atomic::{AtomicU64, Ordering}};
use tokio::task::JoinHandle;

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClientTransportConnected {
    pub server: SocketAddr,
    pub connection_id: u64,
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClientTransportDisconnected {
    pub server: SocketAddr,
    pub connection_id: u64,
}

#[derive(Resource, Clone, Default)]
pub struct ClientTransportConnectionIds(Arc<AtomicU64>);

impl ClientTransportConnectionIds {
    pub fn next(&self) -> u64 {
        self.0.fetch_add(1, Ordering::Relaxed).wrapping_add(1)
    }
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
            .init_resource::<ClientTransportConnectionIds>()
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
