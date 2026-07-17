use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_player_scale_map_api::{ClientPlayerScaleMapApi, ClientPlayerScaleMapSet};
use player_hitbox_api::{PlayerHitbox, PlayerHitboxApi};
use player_scale_api::{PlayerScale, PlayerScaleApi};
use tokio::task::JoinHandle;

pub struct ClientPlayerHitboxScaleVanillaMod;

impl ClientPlayerHitboxScaleVanillaMod {
    pub fn init<H: PlayerHitboxApi, S: PlayerScaleApi, M: ClientPlayerScaleMapApi>(
        bevy: &mut BevyMod,
        _hitbox: &mut H,
        _scale: &mut S,
        _scale_map: &mut M,
    ) -> Self {
        bevy.app
            .add_systems(Update, apply_scaled_hitbox.after(ClientPlayerScaleMapSet));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn apply_scaled_hitbox(scale: Res<PlayerScale>, mut hitbox: ResMut<PlayerHitbox>) {
    if !scale.is_changed() {
        return;
    }
    *hitbox = PlayerHitbox::default().scaled(scale.0);
}
