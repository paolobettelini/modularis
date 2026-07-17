use bevy::prelude::*;
use player_hitbox_api::PlayerHitbox;
use player_network_message_types::PlayerId;
use std::collections::HashMap;

#[derive(Resource, Debug, Default)]
pub struct ServerPlayerHitboxes {
    overrides: HashMap<PlayerId, PlayerHitbox>,
}

impl ServerPlayerHitboxes {
    pub fn hitbox(&self, player_id: PlayerId) -> PlayerHitbox {
        self.overrides.get(&player_id).copied().unwrap_or_default()
    }

    pub fn set(&mut self, player_id: PlayerId, hitbox: PlayerHitbox) -> bool {
        let previous = self.hitbox(player_id);
        if hitbox == PlayerHitbox::default() {
            self.overrides.remove(&player_id);
        } else {
            self.overrides.insert(player_id, hitbox);
        }
        previous != hitbox
    }

    pub fn remove(&mut self, player_id: PlayerId) {
        self.overrides.remove(&player_id);
    }
}

#[derive(Message, Debug, Clone, Copy, PartialEq)]
pub struct SetServerPlayerHitbox {
    pub player_id: PlayerId,
    pub hitbox: PlayerHitbox,
}

#[derive(Message, Debug, Clone, Copy, PartialEq)]
pub struct ServerPlayerHitboxChanged {
    pub player_id: PlayerId,
    pub hitbox: PlayerHitbox,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ServerPlayerHitboxSet;

pub trait ServerPlayerHitboxApi: Send + Sync + 'static {}
