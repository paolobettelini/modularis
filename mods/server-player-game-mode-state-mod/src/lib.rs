use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_player_game_mode_api::{
    ServerPlayerGameModeApi, ServerPlayerGameModeChanged, ServerPlayerGameModeSet,
    ServerPlayerGameModes, SetPlayerGameMode,
};
use server_player_lifecycle_events_api::ServerPlayerLeft;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use server_player_registry_api::ServerPlayerSessionSet;
use tokio::task::JoinHandle;

pub struct ServerPlayerGameModeStateMod;

impl ServerPlayerGameModeStateMod {
    pub fn init(bevy: &mut BevyMod, _lifecycle: &mut ServerPlayerLifecycleEventsMod) -> Self {
        bevy.app
            .init_resource::<ServerPlayerGameModes>()
            .add_message::<SetPlayerGameMode>()
            .add_message::<ServerPlayerGameModeChanged>()
            .configure_sets(
                Update,
                (ServerPlayerGameModeSet::Apply, ServerPlayerGameModeSet::ApplyPolicy)
                    .chain()
                    .after(ServerPlayerSessionSet::Initialize),
            )
            .add_systems(Update, apply_modes.in_set(ServerPlayerGameModeSet::Apply))
            .add_systems(Update, remove_left_players.after(ServerPlayerGameModeSet::ApplyPolicy));
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

impl ServerPlayerGameModeApi for ServerPlayerGameModeStateMod {}

fn apply_modes(
    mut modes: ResMut<ServerPlayerGameModes>,
    mut requests: MessageReader<SetPlayerGameMode>,
    mut changed: MessageWriter<ServerPlayerGameModeChanged>,
) {
    for request in requests.read() {
        if modes.get(request.player_id) == Some(request.mode) { continue; }
        let previous = modes.set(request.player_id, request.mode);
        changed.write(ServerPlayerGameModeChanged { player_id: request.player_id, previous, mode: request.mode });
    }
}

fn remove_left_players(
    mut modes: ResMut<ServerPlayerGameModes>,
    mut left: MessageReader<ServerPlayerLeft>,
) {
    for event in left.read() { modes.remove(event.player_id); }
}
