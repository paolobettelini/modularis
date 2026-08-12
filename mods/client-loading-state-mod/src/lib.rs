use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameState, GameStateApi};
use client_loading_api::{
    ClearClientLoadingAuthority, ClientLoadingApi, ClientLoadingSet, ClientLoadingTaskChanged,
    ClientLoadingTasks, RemoveClientLoadingTask, SetClientLoadingTask,
};
use tokio::task::JoinHandle;

pub struct ClientLoadingStateMod;

impl ClientLoadingStateMod {
    pub fn init<G: GameStateApi>(bevy: &mut BevyMod, _game: &mut G) -> Self {
        bevy.app
            .init_resource::<ClientLoadingTasks>()
            .add_message::<SetClientLoadingTask>()
            .add_message::<RemoveClientLoadingTask>()
            .add_message::<ClearClientLoadingAuthority>()
            .add_message::<ClientLoadingTaskChanged>()
            .configure_sets(
                Update,
                (
                    ClientLoadingSet::Receive,
                    ClientLoadingSet::Apply,
                    ClientLoadingSet::React,
                    ClientLoadingSet::Render,
                ).chain(),
            )
            .add_systems(
                Update,
                (clear_authorities, remove_tasks, set_tasks)
                    .chain()
                    .in_set(ClientLoadingSet::Apply),
            )
            .add_systems(OnExit(GameState::InGame), clear_all_tasks);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

impl ClientLoadingApi for ClientLoadingStateMod {}

fn set_tasks(
    mut state: ResMut<ClientLoadingTasks>,
    mut requests: MessageReader<SetClientLoadingTask>,
    mut changed: MessageWriter<ClientLoadingTaskChanged>,
) {
    for request in requests.read() {
        let key = client_loading_api::ClientLoadingTaskKey {
            authority: request.authority.clone(),
            id: request.task.id.clone(),
        };
        if state.set(key.clone(), request.task.clone()) {
            changed.write(ClientLoadingTaskChanged { key, task: Some(request.task.clone().normalized()) });
        }
    }
}

fn remove_tasks(
    mut state: ResMut<ClientLoadingTasks>,
    mut requests: MessageReader<RemoveClientLoadingTask>,
    mut changed: MessageWriter<ClientLoadingTaskChanged>,
) {
    for request in requests.read() {
        if state.remove(&request.key) {
            changed.write(ClientLoadingTaskChanged { key: request.key.clone(), task: None });
        }
    }
}

fn clear_authorities(
    mut state: ResMut<ClientLoadingTasks>,
    mut requests: MessageReader<ClearClientLoadingAuthority>,
    mut changed: MessageWriter<ClientLoadingTaskChanged>,
) {
    for request in requests.read() {
        for key in state.keys_for_authority(&request.authority) {
            state.remove(&key);
            changed.write(ClientLoadingTaskChanged { key, task: None });
        }
    }
}

fn clear_all_tasks(mut state: ResMut<ClientLoadingTasks>) {
    state.clear();
}
