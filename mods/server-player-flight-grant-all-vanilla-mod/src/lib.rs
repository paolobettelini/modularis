use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_player_flight_api::{ServerPlayerFlightApi, SetPlayerFlightCapability};
use server_player_lifecycle_events_api::ServerPlayerJoined;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use tokio::task::JoinHandle;

pub struct ServerPlayerFlightGrantAllVanillaMod;

impl ServerPlayerFlightGrantAllVanillaMod {
    pub fn init<F: ServerPlayerFlightApi>(
        bevy: &mut BevyMod,
        _flight: &mut F,
        _lifecycle: &mut ServerPlayerLifecycleEventsMod,
    ) -> Self {
        bevy.app.add_systems(Update, grant_flight_on_join);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn grant_flight_on_join(
    mut joined: MessageReader<ServerPlayerJoined>,
    mut capabilities: MessageWriter<SetPlayerFlightCapability>,
) {
    for player in joined.read() {
        capabilities.write(SetPlayerFlightCapability {
            player_id: player.player_id,
            enabled: true,
        });
    }
}
