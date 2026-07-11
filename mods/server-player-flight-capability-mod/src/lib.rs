use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_player_flight_api::{
    ServerPlayerFlightApi, ServerPlayerFlightCapabilities, ServerPlayerFlightCapabilityChanged,
    ServerPlayerFlightSet, SetPlayerFlightCapability,
};
use server_player_lifecycle_events_api::ServerPlayerLeft;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use tokio::task::JoinHandle;

pub struct ServerPlayerFlightCapabilityMod;

impl ServerPlayerFlightCapabilityMod {
    pub fn init(bevy: &mut BevyMod, _lifecycle: &mut ServerPlayerLifecycleEventsMod) -> Self {
        bevy.app
            .init_resource::<ServerPlayerFlightCapabilities>()
            .add_message::<SetPlayerFlightCapability>()
            .add_message::<ServerPlayerFlightCapabilityChanged>()
            .configure_sets(
                Update,
                (ServerPlayerFlightSet::Apply, ServerPlayerFlightSet::Sync).chain(),
            )
            .add_systems(
                Update,
                apply_capability_changes.in_set(ServerPlayerFlightSet::Apply),
            )
            .add_systems(Update, remove_left_players);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerPlayerFlightApi for ServerPlayerFlightCapabilityMod {}

fn apply_capability_changes(
    mut capabilities: ResMut<ServerPlayerFlightCapabilities>,
    mut requests: MessageReader<SetPlayerFlightCapability>,
    mut changed: MessageWriter<ServerPlayerFlightCapabilityChanged>,
) {
    for request in requests.read() {
        if capabilities.enabled(request.player_id) == request.enabled {
            continue;
        }
        capabilities.set(request.player_id, request.enabled);
        changed.write(ServerPlayerFlightCapabilityChanged {
            player_id: request.player_id,
            enabled: request.enabled,
        });
    }
}

fn remove_left_players(
    mut capabilities: ResMut<ServerPlayerFlightCapabilities>,
    mut left: MessageReader<ServerPlayerLeft>,
) {
    for player in left.read() {
        capabilities.remove(player.player_id);
    }
}
