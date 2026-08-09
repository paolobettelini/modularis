use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_permission_registry::PermissionId;
use server_player_flight_api::{ServerPlayerFlightApi, ServerPlayerFlightSet, SetPlayerFlightCapability};
use server_player_permission_api::{
    ServerPlayerPermissionApi, ServerPlayerPermissionSet, ServerPlayerPermissionsChanged,
};
use tokio::task::JoinHandle;

pub struct ServerPlayerFlightPermissionVanillaMod;

impl ServerPlayerFlightPermissionVanillaMod {
    pub fn init<P: ServerPlayerPermissionApi, F: ServerPlayerFlightApi>(
        bevy: &mut BevyMod,
        _permissions: &mut P,
        _flight: &mut F,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            derive_flight_capability
                .in_set(ServerPlayerPermissionSet::DeriveCapabilities)
                .before(ServerPlayerFlightSet::Apply),
        );
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

fn derive_flight_capability(
    mut changes: MessageReader<ServerPlayerPermissionsChanged>,
    mut flight: MessageWriter<SetPlayerFlightCapability>,
) {
    for change in changes.read() {
        if change.added.contains(&PermissionId::CanFlight) {
            flight.write(SetPlayerFlightCapability { player_id: change.player_id, enabled: true });
        } else if change.removed.contains(&PermissionId::CanFlight) {
            flight.write(SetPlayerFlightCapability { player_id: change.player_id, enabled: false });
        }
    }
}
