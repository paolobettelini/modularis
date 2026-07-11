use bevy::prelude::*;
use generated_network_messages::ClientBoundMessage;
use std::{
    collections::HashSet,
    io,
    net::SocketAddr,
    sync::{Arc, RwLock},
};

#[derive(Resource, Clone)]
pub struct ServerNetworkSender {
    send: Arc<dyn Fn(SocketAddr, &ClientBoundMessage) -> io::Result<()> + Send + Sync>,
    clients: Arc<RwLock<HashSet<SocketAddr>>>,
}

impl ServerNetworkSender {
    pub fn new<S>(send: S) -> Self
    where
        S: Fn(SocketAddr, &ClientBoundMessage) -> io::Result<()> + Send + Sync + 'static,
    {
        Self {
            send: Arc::new(send),
            clients: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    pub fn register_client(&self, address: SocketAddr) {
        self.clients
            .write()
            .expect("server clients lock poisoned")
            .insert(address);
    }

    pub fn remove_client(&self, address: SocketAddr) {
        self.clients
            .write()
            .expect("server clients lock poisoned")
            .remove(&address);
    }

    pub fn send_to(&self, address: SocketAddr, message: &ClientBoundMessage) -> io::Result<()> {
        (self.send)(address, message)
    }

    pub fn broadcast(&self, message: &ClientBoundMessage) {
        let clients = self
            .clients
            .read()
            .expect("server clients lock poisoned")
            .iter()
            .copied()
            .collect::<Vec<_>>();
        for client in clients {
            if let Err(error) = self.send_to(client, message) {
                warn!("failed to send packet to {client}: {error}");
            }
        }
    }

    pub fn broadcast_except(&self, excluded: SocketAddr, message: &ClientBoundMessage) {
        let clients = self
            .clients
            .read()
            .expect("server clients lock poisoned")
            .iter()
            .copied()
            .filter(|address| *address != excluded)
            .collect::<Vec<_>>();
        for client in clients {
            if let Err(error) = self.send_to(client, message) {
                warn!("failed to send packet to {client}: {error}");
            }
        }
    }
}

pub trait ServerNetworkApi: Send + Sync + 'static {}
