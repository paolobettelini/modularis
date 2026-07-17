use bevy::prelude::*;
use bevy_mod::BevyMod;
use player_gravity_api::{Gravity, PlayerGravityApi};
use server_player_gravity_api::{
    ServerPlayerGravities, ServerPlayerGravityApi, ServerPlayerGravityChanged,
    ServerPlayerGravitySet, SetServerPlayerGravity,
};
use server_player_lifecycle_events_api::ServerPlayerLeft;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use tokio::task::JoinHandle;

pub struct ServerPlayerGravityStateMod;

impl ServerPlayerGravityStateMod {
    pub fn init<G: PlayerGravityApi>(
        bevy: &mut BevyMod,
        _gravity: &mut G,
        _lifecycle: &mut ServerPlayerLifecycleEventsMod,
    ) -> Self {
        let default = bevy.app.world().resource::<Gravity>().0;
        bevy.app
            .insert_resource(ServerPlayerGravities::new(default))
            .add_message::<SetServerPlayerGravity>()
            .add_message::<ServerPlayerGravityChanged>()
            .configure_sets(
                Update,
                (ServerPlayerGravitySet::Apply, ServerPlayerGravitySet::Sync).chain(),
            )
            .add_systems(
                Update,
                apply_gravity_changes.in_set(ServerPlayerGravitySet::Apply),
            )
            .add_systems(Update, remove_left_players);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerPlayerGravityApi for ServerPlayerGravityStateMod {}

fn apply_gravity_changes(
    mut gravities: ResMut<ServerPlayerGravities>,
    mut requests: MessageReader<SetServerPlayerGravity>,
    mut changed: MessageWriter<ServerPlayerGravityChanged>,
) {
    for request in requests.read() {
        if !request.gravity.is_finite() {
            continue;
        }
        if gravities.set(request.player_id, request.gravity) {
            changed.write(ServerPlayerGravityChanged {
                player_id: request.player_id,
                gravity: request.gravity,
            });
        }
    }
}

fn remove_left_players(
    mut gravities: ResMut<ServerPlayerGravities>,
    mut left: MessageReader<ServerPlayerLeft>,
) {
    for player in left.read() {
        gravities.remove(player.player_id);
    }
}
