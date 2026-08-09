use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_player_lifecycle_events_api::ServerPlayerLeft;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use server_player_outline_api::{
    ServerPlayerOutlineApi, ServerPlayerOutlineChanged, ServerPlayerOutlineSet,
    ServerPlayerOutlines, SetPlayerOutline,
};
use tokio::task::JoinHandle;

pub struct ServerPlayerOutlineStateMod;

impl ServerPlayerOutlineStateMod {
    pub fn init(bevy: &mut BevyMod, _lifecycle: &mut ServerPlayerLifecycleEventsMod) -> Self {
        bevy.app
            .init_resource::<ServerPlayerOutlines>()
            .add_message::<SetPlayerOutline>()
            .add_message::<ServerPlayerOutlineChanged>()
            .configure_sets(Update, (ServerPlayerOutlineSet::Apply, ServerPlayerOutlineSet::Sync).chain())
            .add_systems(Update, apply_changes.in_set(ServerPlayerOutlineSet::Apply))
            .add_systems(Update, remove_left_players.after(ServerPlayerOutlineSet::Sync));
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

impl ServerPlayerOutlineApi for ServerPlayerOutlineStateMod {}

fn apply_changes(
    mut state: ResMut<ServerPlayerOutlines>,
    mut requests: MessageReader<SetPlayerOutline>,
    mut changed: MessageWriter<ServerPlayerOutlineChanged>,
) {
    for request in requests.read() {
        if state.set(request.player_id, request.enabled) {
            changed.write(ServerPlayerOutlineChanged { player_id: request.player_id, enabled: request.enabled });
        }
    }
}

fn remove_left_players(
    mut state: ResMut<ServerPlayerOutlines>,
    mut left: MessageReader<ServerPlayerLeft>,
) {
    for event in left.read() { state.remove(event.player_id); }
}
