use bevy::prelude::*;
use bevy_mod::BevyMod;
use loading_task_api::LoadingTask;
use server_chunk_stream_events_api::ServerChunkSent;
use server_chunk_stream_events_mod::ServerChunkStreamEventsMod;
use server_loading_api::{
    RemoveServerPlayerLoadingTask, ServerLoadingApi, ServerLoadingTaskKey,
    SetServerPlayerLoadingTask,
};
use server_player_lifecycle_events_api::{ServerPlayerJoined, ServerPlayerLeft, ServerPlayerReady};
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use std::collections::{HashMap, HashSet};
use tokio::task::JoinHandle;

const AUTHORITY: &str = "vanilla:server-join";
const TASK_ID: &str = "vanilla:initial-world-stream";

#[derive(Resource, Default)]
struct CompletedJoinLoading {
    completed_players: HashSet<u64>,
    removal_deadlines: HashMap<u64, f64>,
}

pub struct ServerJoinLoadingVanillaMod;

impl ServerJoinLoadingVanillaMod {
    pub fn init<L: ServerLoadingApi>(
        bevy: &mut BevyMod,
        _loading: &mut L,
        _lifecycle: &mut ServerPlayerLifecycleEventsMod,
        _stream: &mut ServerChunkStreamEventsMod,
    ) -> Self {
        bevy.app
            .init_resource::<CompletedJoinLoading>()
            .add_systems(
                Update,
                (
                    begin_server_join_loading,
                    mark_session_ready,
                    mark_first_chunk_ready,
                    remove_completed_join_loading,
                    cleanup_left_players,
                ),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

fn publish(
    player_id: u64,
    progress: f32,
    status: &str,
    tasks: &mut MessageWriter<SetServerPlayerLoadingTask>,
) {
    tasks.write(SetServerPlayerLoadingTask {
        player_id,
        authority: AUTHORITY.to_string(),
        task: LoadingTask::new(TASK_ID, "Server world preparation", progress, status),
    });
}

fn begin_server_join_loading(
    mut joined: MessageReader<ServerPlayerJoined>,
    mut tasks: MessageWriter<SetServerPlayerLoadingTask>,
) {
    for event in joined.read() {
        publish(event.player_id, 0.15, "Preparing the player session", &mut tasks);
    }
}

fn mark_session_ready(
    mut ready: MessageReader<ServerPlayerReady>,
    mut tasks: MessageWriter<SetServerPlayerLoadingTask>,
) {
    for event in ready.read() {
        publish(event.player_id, 0.55, "Resolving world scope and chunk stream", &mut tasks);
    }
}

fn mark_first_chunk_ready(
    time: Res<Time>,
    mut chunks: MessageReader<ServerChunkSent>,
    mut completed: ResMut<CompletedJoinLoading>,
    mut tasks: MessageWriter<SetServerPlayerLoadingTask>,
) {
    for event in chunks.read() {
        if !completed.completed_players.insert(event.player_id) { continue; }
        publish(event.player_id, 1.0, "Initial world data sent", &mut tasks);
        completed.removal_deadlines.insert(event.player_id, time.elapsed_secs_f64() + 0.5);
    }
}

fn remove_completed_join_loading(
    time: Res<Time>,
    mut completed: ResMut<CompletedJoinLoading>,
    mut removals: MessageWriter<RemoveServerPlayerLoadingTask>,
) {
    let now = time.elapsed_secs_f64();
    let players = completed.removal_deadlines.iter().filter(|(_, deadline)| now >= **deadline)
        .map(|(player_id, _)| *player_id).collect::<Vec<_>>();
    for player_id in players {
        completed.removal_deadlines.remove(&player_id);
        removals.write(RemoveServerPlayerLoadingTask {
            key: ServerLoadingTaskKey {
                player_id,
                authority: AUTHORITY.to_string(),
                id: TASK_ID.to_string(),
            },
        });
    }
}

fn cleanup_left_players(
    mut left: MessageReader<ServerPlayerLeft>,
    mut completed: ResMut<CompletedJoinLoading>,
) {
    for event in left.read() {
        completed.completed_players.remove(&event.player_id);
        completed.removal_deadlines.remove(&event.player_id);
    }
}
