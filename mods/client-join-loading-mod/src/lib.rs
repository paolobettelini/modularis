use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_chunk_cache_api::{ClientChunkAvailable, ClientChunkCache, ClientChunkCacheApi};
use client_game_state_api::{GameState, GameStateApi};
use client_loading_api::{
    ClearClientLoadingAuthority, ClientLoadingApi, ClientLoadingAuthority, ClientLoadingSet,
    ClientLoadingTaskKey, RemoveClientLoadingTask, SetClientLoadingTask,
};
use client_session_api::{ClientSession, ClientSessionApi};
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
    connection_id: Option<u64>,
    session_accepted: bool,
    remove_after: Option<f64>,
}

pub struct ClientJoinLoadingMod;

impl ClientJoinLoadingMod {
    pub fn init<
        L: ClientLoadingApi,
        G: GameStateApi,
        C: ClientChunkCacheApi,
        S: ClientSessionApi,
    >(
        bevy: &mut BevyMod,
        _loading: &mut L,
        _game: &mut G,
        _chunks: &mut C,
        _session: &mut S,
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
    state.connection_id = None;
    state.session_accepted = false;
    state.remove_after = None;
    publish(0.05, "Connecting to the server", &mut tasks);
}

fn observe_connection(
    mut connected: MessageReader<ClientTransportConnected>,
    mut state: ResMut<ClientJoinLoadingState>,
    mut tasks: MessageWriter<SetClientLoadingTask>,
) {
    if let Some(connected) = connected.read().last().filter(|_| state.active) {
        state.connection_id = Some(connected.connection_id);
        publish(0.25, "Transport connected; authenticating session", &mut tasks);
    }
}

fn observe_join_acceptance(
    mut accepted: MessageReader<JoinAcceptedReceived>,
    session: Res<ClientSession>,
    mut state: ResMut<ClientJoinLoadingState>,
    mut tasks: MessageWriter<SetClientLoadingTask>,
) {
    let received = accepted.read().last().is_some();
    if state.active && !state.session_accepted && (received || session.player_id.is_some()) {
        state.session_accepted = true;
        publish(0.65, "Session accepted; receiving world data", &mut tasks);
    }
}

fn observe_first_chunk(
    time: Res<Time>,
    mut available: MessageReader<ClientChunkAvailable>,
    chunks: Res<ClientChunkCache>,
    mut state: ResMut<ClientJoinLoadingState>,
    mut tasks: MessageWriter<SetClientLoadingTask>,
) {
    let received = available.read().next().is_some();
    if !state.active || state.remove_after.is_some() || (!received && chunks.is_empty()) {
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
    state.connection_id = None;
    state.session_accepted = false;
    state.remove_after = None;
}

fn cancel_disconnected_join(
    mut disconnected: MessageReader<ClientTransportDisconnected>,
    mut state: ResMut<ClientJoinLoadingState>,
    mut clear: MessageWriter<ClearClientLoadingAuthority>,
) {
    let disconnected_active_connection = disconnected
        .read()
        .any(|event| Some(event.connection_id) == state.connection_id);
    if !disconnected_active_connection { return; }
    state.active = false;
    state.connection_id = None;
    state.session_accepted = false;
    state.remove_after = None;
    clear.write(ClearClientLoadingAuthority { authority: authority() });
}
