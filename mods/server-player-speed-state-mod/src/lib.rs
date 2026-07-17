use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_player_lifecycle_events_api::ServerPlayerLeft;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use server_player_speed_api::{
    ServerPlayerSpeedApi, ServerPlayerSpeedChanged, ServerPlayerSpeedSet, ServerPlayerSpeeds,
    SetServerPlayerSpeed,
};
use tokio::task::JoinHandle;

pub struct ServerPlayerSpeedStateMod;

impl ServerPlayerSpeedStateMod {
    pub fn init(bevy: &mut BevyMod, _lifecycle: &mut ServerPlayerLifecycleEventsMod) -> Self {
        bevy.app
            .init_resource::<ServerPlayerSpeeds>()
            .add_message::<SetServerPlayerSpeed>()
            .add_message::<ServerPlayerSpeedChanged>()
            .configure_sets(
                Update,
                (ServerPlayerSpeedSet::Apply, ServerPlayerSpeedSet::Sync).chain(),
            )
            .add_systems(
                Update,
                apply_speed_changes.in_set(ServerPlayerSpeedSet::Apply),
            )
            .add_systems(Update, remove_left_players);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerPlayerSpeedApi for ServerPlayerSpeedStateMod {}

fn apply_speed_changes(
    mut speeds: ResMut<ServerPlayerSpeeds>,
    mut requests: MessageReader<SetServerPlayerSpeed>,
    mut changed: MessageWriter<ServerPlayerSpeedChanged>,
) {
    for request in requests.read() {
        if !request.multiplier.is_finite() {
            continue;
        }
        let multiplier = request.multiplier.max(0.0);
        if speeds.set(request.player_id, multiplier) {
            changed.write(ServerPlayerSpeedChanged {
                player_id: request.player_id,
                multiplier,
            });
        }
    }
}

fn remove_left_players(
    mut speeds: ResMut<ServerPlayerSpeeds>,
    mut left: MessageReader<ServerPlayerLeft>,
) {
    for player in left.read() {
        speeds.remove(player.player_id);
    }
}
