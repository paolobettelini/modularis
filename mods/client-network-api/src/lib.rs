use bevy::prelude::*;
use generated_network_messages::ServerBoundMessage;
use std::{io, sync::Arc};

#[derive(Resource, Clone)]
pub struct ClientNetworkSender {
    send: Arc<dyn Fn(&ServerBoundMessage) -> io::Result<()> + Send + Sync>,
}

impl ClientNetworkSender {
    pub fn new<S>(send: S) -> Self
    where
        S: Fn(&ServerBoundMessage) -> io::Result<()> + Send + Sync + 'static,
    {
        Self {
            send: Arc::new(send),
        }
    }

    pub fn send(&self, message: &ServerBoundMessage) -> io::Result<()> {
        (self.send)(message)
    }
}

pub trait ClientNetworkApi: Send + Sync + 'static {}
