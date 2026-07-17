use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_player_flight_speed_api::{
    ServerPlayerFlightSpeedApi, ServerPlayerFlightSpeedChanged, ServerPlayerFlightSpeedSet,
    ServerPlayerFlightSpeeds, SetServerPlayerFlightSpeed,
};
use server_player_lifecycle_events_api::ServerPlayerLeft;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use tokio::task::JoinHandle;

pub struct ServerPlayerFlightSpeedStateMod;

impl ServerPlayerFlightSpeedStateMod {
    pub fn init(bevy: &mut BevyMod, _lifecycle: &mut ServerPlayerLifecycleEventsMod) -> Self {
        bevy.app
            .init_resource::<ServerPlayerFlightSpeeds>()
            .add_message::<SetServerPlayerFlightSpeed>()
            .add_message::<ServerPlayerFlightSpeedChanged>()
            .configure_sets(
                Update,
                (
                    ServerPlayerFlightSpeedSet::Apply,
                    ServerPlayerFlightSpeedSet::Sync,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                apply_flight_speed_changes.in_set(ServerPlayerFlightSpeedSet::Apply),
            )
            .add_systems(Update, remove_left_players);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerPlayerFlightSpeedApi for ServerPlayerFlightSpeedStateMod {}

fn apply_flight_speed_changes(
    mut speeds: ResMut<ServerPlayerFlightSpeeds>,
    mut requests: MessageReader<SetServerPlayerFlightSpeed>,
    mut changed: MessageWriter<ServerPlayerFlightSpeedChanged>,
) {
    for request in requests.read() {
        if !request.multiplier.is_finite() {
            continue;
        }
        let multiplier = request.multiplier.max(0.0);
        if speeds.set(request.player_id, multiplier) {
            changed.write(ServerPlayerFlightSpeedChanged {
                player_id: request.player_id,
                multiplier,
            });
        }
    }
}

fn remove_left_players(
    mut speeds: ResMut<ServerPlayerFlightSpeeds>,
    mut left: MessageReader<ServerPlayerLeft>,
) {
    for player in left.read() {
        speeds.remove(player.player_id);
    }
}
