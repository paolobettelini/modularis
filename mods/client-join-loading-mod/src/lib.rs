use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_chunk_cache_api::{ClientChunkAvailable, ClientChunkCacheApi};
use client_game_state_api::{GameState, GameStateApi};
use client_loading_api::{
    ClearClientLoadingAuthority, ClientLoadingApi, ClientLoadingAuthority, ClientLoadingSet,
    ClientLoadingTaskKey, RemoveClientLoadingTask, SetClientLoadingTask,
};
use generated_network_messages::{JoinAcceptedReceived, NetworkMessageSet};
use loading_task_api::LoadingTask;
use network_protocol_mod::NetworkProtocolMod;
use network_transport_events_mod::{
    ClientTransportConnected, ClientTransportDisconnected, NetworkTransportEventsMod,
};
use tokio::task::JoinHandle;

const AUTHORITY: &str = "modularis:client-join";
const TASK_ID: &str = "modularis:server-join";

#[derive(Resource, Default)]
struct ClientJoinLoadingState {
    active: bool,
    remove_after: Option<f64>,
}

pub struct ClientJoinLoadingMod;

impl ClientJoinLoadingMod {
    pub fn init<L: ClientLoadingApi, G: GameStateApi, C: ClientChunkCacheApi>(
        bevy: &mut BevyMod,
        _loading: &mut L,
        _game: &mut G,
        _chunks: &mut C,
        _transport: &mut NetworkTransportEventsMod,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app
            .init_resource::<ClientJoinLoadingState>()
            .add_systems(OnEnter(GameState::InGame), begin_join_loading)
            .add_systems(
                Update,
                (
                    observe_connection,
                    observe_join_acceptance,
                    observe_first_chunk,
                    finish_join_loading,
                    cancel_disconnected_join,
                )
                    .chain()
                    .after(NetworkMessageSet::DispatchPackets)
                    .in_set(ClientLoadingSet::React)
                    .run_if(in_state(GameState::InGame)),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

fn authority() -> ClientLoadingAuthority {
    ClientLoadingAuthority::Local(AUTHORITY.to_string())
}

fn publish(progress: f32, status: &str, tasks: &mut MessageWriter<SetClientLoadingTask>) {
    tasks.write(SetClientLoadingTask {
        authority: authority(),
        task: LoadingTask::new(TASK_ID, "Joining server", progress, status),
    });
}

fn begin_join_loading(
    mut state: ResMut<ClientJoinLoadingState>,
    mut tasks: MessageWriter<SetClientLoadingTask>,
) {
    state.active = true;
    state.remove_after = None;
    publish(0.05, "Connecting to the server", &mut tasks);
}

fn observe_connection(
    mut connected: MessageReader<ClientTransportConnected>,
    state: Res<ClientJoinLoadingState>,
    mut tasks: MessageWriter<SetClientLoadingTask>,
) {
    if state.active && connected.read().last().is_some() {
        publish(0.25, "Transport connected; authenticating session", &mut tasks);
    }
}

fn observe_join_acceptance(
    mut accepted: MessageReader<JoinAcceptedReceived>,
    state: Res<ClientJoinLoadingState>,
    mut tasks: MessageWriter<SetClientLoadingTask>,
) {
    if state.active && accepted.read().last().is_some() {
        publish(0.65, "Session accepted; receiving world data", &mut tasks);
    }
}

fn observe_first_chunk(
    time: Res<Time>,
    mut available: MessageReader<ClientChunkAvailable>,
    mut state: ResMut<ClientJoinLoadingState>,
    mut tasks: MessageWriter<SetClientLoadingTask>,
) {
    if !state.active || state.remove_after.is_some() || available.read().next().is_none() {
        return;
    }
    publish(1.0, "World stream is ready", &mut tasks);
    state.remove_after = Some(time.elapsed_secs_f64() + 0.35);
}

fn finish_join_loading(
    time: Res<Time>,
    mut state: ResMut<ClientJoinLoadingState>,
    mut tasks: MessageWriter<RemoveClientLoadingTask>,
) {
    if !state.remove_after.is_some_and(|deadline| time.elapsed_secs_f64() >= deadline) {
        return;
    }
    tasks.write(RemoveClientLoadingTask {
        key: ClientLoadingTaskKey { authority: authority(), id: TASK_ID.to_string() },
    });
    state.active = false;
    state.remove_after = None;
}

fn cancel_disconnected_join(
    mut disconnected: MessageReader<ClientTransportDisconnected>,
    mut state: ResMut<ClientJoinLoadingState>,
    mut clear: MessageWriter<ClearClientLoadingAuthority>,
) {
    if disconnected.read().last().is_none() { return; }
    state.active = false;
    state.remove_after = None;
    clear.write(ClearClientLoadingAuthority { authority: authority() });
}
