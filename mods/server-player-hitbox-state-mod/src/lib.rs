use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_player_hitbox_api::{
    ServerPlayerHitboxApi, ServerPlayerHitboxChanged, ServerPlayerHitboxSet, ServerPlayerHitboxes,
    SetServerPlayerHitbox,
};
use server_player_lifecycle_events_api::ServerPlayerLeft;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use tokio::task::JoinHandle;

pub struct ServerPlayerHitboxStateMod;

impl ServerPlayerHitboxStateMod {
    pub fn init(bevy: &mut BevyMod, _lifecycle: &mut ServerPlayerLifecycleEventsMod) -> Self {
        bevy.app
            .init_resource::<ServerPlayerHitboxes>()
            .add_message::<SetServerPlayerHitbox>()
            .add_message::<ServerPlayerHitboxChanged>()
            .add_systems(Update, apply_hitbox_changes.in_set(ServerPlayerHitboxSet))
            .add_systems(Update, remove_left_players);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerPlayerHitboxApi for ServerPlayerHitboxStateMod {}

fn apply_hitbox_changes(
    mut hitboxes: ResMut<ServerPlayerHitboxes>,
    mut requests: MessageReader<SetServerPlayerHitbox>,
    mut changed: MessageWriter<ServerPlayerHitboxChanged>,
) {
    for request in requests.read() {
        if !request.hitbox.is_valid() {
            continue;
        }
        if hitboxes.set(request.player_id, request.hitbox) {
            changed.write(ServerPlayerHitboxChanged {
                player_id: request.player_id,
                hitbox: request.hitbox,
            });
        }
    }
}

fn remove_left_players(
    mut hitboxes: ResMut<ServerPlayerHitboxes>,
    mut left: MessageReader<ServerPlayerLeft>,
) {
    for player in left.read() {
        hitboxes.remove(player.player_id);
    }
}
