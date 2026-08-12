use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_loading_api::{
    RemoveServerPlayerLoadingTask, ServerLoadingApi, ServerLoadingSet,
    ServerLoadingTaskKey, ServerLoadingTasks, ServerPlayerLoadingTaskChanged,
    SetServerPlayerLoadingTask,
};
use server_player_lifecycle_events_api::ServerPlayerLeft;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use server_player_registry_api::ServerPlayerSessionSet;
use tokio::task::JoinHandle;

pub struct ServerLoadingStateMod;

impl ServerLoadingStateMod {
    pub fn init(bevy: &mut BevyMod, _lifecycle: &mut ServerPlayerLifecycleEventsMod) -> Self {
        bevy.app
            .init_resource::<ServerLoadingTasks>()
            .add_message::<SetServerPlayerLoadingTask>()
            .add_message::<RemoveServerPlayerLoadingTask>()
            .add_message::<ServerPlayerLoadingTaskChanged>()
            .configure_sets(
                Update,
                (ServerLoadingSet::Apply, ServerLoadingSet::Sync)
                    .chain()
                    .after(ServerPlayerSessionSet::Initialize),
            )
            .add_systems(
                Update,
                (remove_tasks, set_tasks).chain().in_set(ServerLoadingSet::Apply),
            )
            .add_systems(Update, remove_left_players.after(ServerLoadingSet::Sync));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

impl ServerLoadingApi for ServerLoadingStateMod {}

fn set_tasks(
    mut state: ResMut<ServerLoadingTasks>,
    mut requests: MessageReader<SetServerPlayerLoadingTask>,
    mut changed: MessageWriter<ServerPlayerLoadingTaskChanged>,
) {
    for request in requests.read() {
        let key = ServerLoadingTaskKey {
            player_id: request.player_id,
            authority: request.authority.clone(),
            id: request.task.id.clone(),
        };
        if state.set(key.clone(), request.task.clone()) {
            changed.write(ServerPlayerLoadingTaskChanged { key, task: Some(request.task.clone().normalized()) });
        }
    }
}

fn remove_tasks(
    mut state: ResMut<ServerLoadingTasks>,
    mut requests: MessageReader<RemoveServerPlayerLoadingTask>,
    mut changed: MessageWriter<ServerPlayerLoadingTaskChanged>,
) {
    for request in requests.read() {
        if state.remove(&request.key) {
            changed.write(ServerPlayerLoadingTaskChanged { key: request.key.clone(), task: None });
        }
    }
}

fn remove_left_players(
    mut state: ResMut<ServerLoadingTasks>,
    mut left: MessageReader<ServerPlayerLeft>,
) {
    for event in left.read() { state.remove_player(event.player_id); }
}
