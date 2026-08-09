use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_permission_registry::PermissionId;
use server_player_lifecycle_events_api::ServerPlayerJoined;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use server_player_permission_api::{ServerPlayerPermissionApi, SetPlayerPermission};
use server_player_registry_api::ServerPlayerSessionSet;
use tokio::task::JoinHandle;

pub struct ServerPlayerFlightGrantAllVanillaMod;

impl ServerPlayerFlightGrantAllVanillaMod {
    pub fn init<P: ServerPlayerPermissionApi>(
        bevy: &mut BevyMod,
        _permissions: &mut P,
        _lifecycle: &mut ServerPlayerLifecycleEventsMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            grant_flight_on_join.in_set(ServerPlayerSessionSet::Initialize),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn grant_flight_on_join(
    mut joined: MessageReader<ServerPlayerJoined>,
    mut permissions: MessageWriter<SetPlayerPermission>,
) {
    for player in joined.read() {
        permissions.write(SetPlayerPermission {
            player_id: player.player_id,
            owner: "vanilla:grant-all-flight".to_string(),
            permission: PermissionId::CanFlight,
            enabled: true,
        });
    }
}
