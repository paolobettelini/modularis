use bevy::prelude::*;
use bevy_mod::BevyMod;
use player_hitbox_api::PlayerHitbox;
use server_player_hitbox_api::{
    ServerPlayerHitboxApi, ServerPlayerHitboxSet, SetServerPlayerHitbox,
};
use server_player_scale_api::{
    ServerPlayerScaleApi, ServerPlayerScaleChanged, ServerPlayerScaleSet,
};
use tokio::task::JoinHandle;

pub struct ServerPlayerHitboxScaleVanillaMod;

impl ServerPlayerHitboxScaleVanillaMod {
    pub fn init<S: ServerPlayerScaleApi, H: ServerPlayerHitboxApi>(
        bevy: &mut BevyMod,
        _scale: &mut S,
        _hitbox: &mut H,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            apply_scaled_hitboxes
                .after(ServerPlayerScaleSet::Apply)
                .before(ServerPlayerScaleSet::Sync)
                .before(ServerPlayerHitboxSet),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn apply_scaled_hitboxes(
    mut scales: MessageReader<ServerPlayerScaleChanged>,
    mut hitboxes: MessageWriter<SetServerPlayerHitbox>,
) {
    for change in scales.read() {
        hitboxes.write(SetServerPlayerHitbox {
            player_id: change.player_id,
            hitbox: PlayerHitbox::default().scaled(change.scale),
        });
    }
}
