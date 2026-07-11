use bevy::prelude::*;
use player_network_message_types::PlayerId;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct NetworkPlayerVisual {
    pub avatar: Entity,
    pub label: Entity,
    pub last_seen_at: f64,
}

#[derive(Resource, Default)]
pub struct RenderedNetworkPlayers {
    pub entities: HashMap<PlayerId, NetworkPlayerVisual>,
}

pub trait ClientPlayerRenderApi: Send + Sync + 'static {}
