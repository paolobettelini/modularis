use bevy::prelude::*;
use player_network_message_types::{NetworkPlayer, PlayerId};
use std::sync::Arc;

#[derive(Resource, Clone)]
pub struct ServerPlayerVisibility {
    can_see: Arc<dyn Fn(&NetworkPlayer, &NetworkPlayer) -> bool + Send + Sync>,
}

impl ServerPlayerVisibility {
    pub fn new<V>(can_see: V) -> Self
    where
        V: Fn(&NetworkPlayer, &NetworkPlayer) -> bool + Send + Sync + 'static,
    {
        Self {
            can_see: Arc::new(can_see),
        }
    }

    pub fn can_see(&self, viewer: &NetworkPlayer, subject: &NetworkPlayer) -> bool {
        (self.can_see)(viewer, subject)
    }

    pub fn viewers_of(&self, subject: &NetworkPlayer, players: &[NetworkPlayer]) -> Vec<PlayerId> {
        players
            .iter()
            .filter(|viewer| viewer.id != subject.id && self.can_see(viewer, subject))
            .map(|viewer| viewer.id)
            .collect()
    }
}

pub trait ServerPlayerVisibilityApi: Send + Sync + 'static {}
