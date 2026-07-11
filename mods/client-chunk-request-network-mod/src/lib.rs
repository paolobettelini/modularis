use bevy::prelude::*;
use bevy_mod::BevyMod;
use chunk_network_message_types::ChunkRequest;
use client_chunk_cache_api::{ClientChunkAvailable, ClientChunkCache, ClientChunkCacheApi};
use client_chunk_streaming_api::{
    ActiveChunks, ChunkNeeded, ChunkStreamingApi, ChunkStreamingFocus, ChunkUnload,
};
use client_chunk_work_priority_api::{ChunkWorkPriorityService, ClientChunkWorkPriorityApi};
use client_game_state_api::{GameState, GameStateApi};
use client_network_api::{ClientNetworkApi, ClientNetworkSender};
use generated_network_messages::ServerBoundMessage;
use std::collections::HashMap;
use tokio::task::JoinHandle;
use voxel_math_api::ChunkPos;

const MAX_REQUESTS_PER_FRAME: usize = 4;
const RETRY_AFTER_SECONDS: f64 = 0.5;

#[derive(Debug, Clone, Copy)]
struct PendingChunkRequest {
    last_sent_at: Option<f64>,
    order: u64,
}

#[derive(Resource, Default)]
struct PendingChunkRequests {
    entries: HashMap<ChunkPos, PendingChunkRequest>,
    next_order: u64,
}

impl PendingChunkRequests {
    fn ensure(&mut self, position: ChunkPos) {
        if self.entries.contains_key(&position) {
            return;
        }
        let order = self.next_order;
        self.next_order = self.next_order.wrapping_add(1);
        self.entries.insert(
            position,
            PendingChunkRequest {
                last_sent_at: None,
                order,
            },
        );
    }
}

pub struct ClientChunkRequestNetworkMod;

impl ClientChunkRequestNetworkMod {
    pub fn init<
        N: ClientNetworkApi,
        S: ChunkStreamingApi,
        C: ClientChunkCacheApi,
        P: ClientChunkWorkPriorityApi,
        G: GameStateApi,
    >(
        bevy: &mut BevyMod,
        _network: &mut N,
        _streaming: &mut S,
        _cache: &mut C,
        _priority: &mut P,
        _game_state: &mut G,
    ) -> Self {
        bevy.app
            .init_resource::<PendingChunkRequests>()
            .add_systems(
                Update,
                (
                    track_needed_chunks,
                    finish_chunk_requests,
                    forget_unloaded_chunks,
                    reconcile_active_chunks,
                    send_chunk_requests,
                )
                    .chain()
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnExit(GameState::InGame), clear_pending_requests);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn track_needed_chunks(
    mut needed: MessageReader<ChunkNeeded>,
    mut pending: ResMut<PendingChunkRequests>,
) {
    for needed in needed.read() {
        pending.ensure(needed.position);
    }
}

fn finish_chunk_requests(
    mut available: MessageReader<ClientChunkAvailable>,
    mut pending: ResMut<PendingChunkRequests>,
) {
    for available in available.read() {
        pending.entries.remove(&available.position);
    }
}

fn forget_unloaded_chunks(
    mut unloads: MessageReader<ChunkUnload>,
    mut pending: ResMut<PendingChunkRequests>,
) {
    for unload in unloads.read() {
        pending.entries.remove(&unload.position);
    }
}

fn reconcile_active_chunks(
    active: Res<ActiveChunks>,
    cache: Res<ClientChunkCache>,
    mut pending: ResMut<PendingChunkRequests>,
) {
    pending
        .entries
        .retain(|position, _| active.positions.contains(position));
    for position in cache.missing_from(&active.positions) {
        pending.ensure(position);
    }
}

fn send_chunk_requests(
    time: Res<Time>,
    sender: Option<Res<ClientNetworkSender>>,
    focus: Res<ChunkStreamingFocus>,
    priority: Res<ChunkWorkPriorityService>,
    mut pending: ResMut<PendingChunkRequests>,
) {
    let Some(sender) = sender else {
        return;
    };

    let now = time.elapsed_secs_f64();
    let mut ready = pending
        .entries
        .iter()
        .filter_map(|(position, request)| {
            request
                .last_sent_at
                .is_none_or(|last_sent| now - last_sent >= RETRY_AFTER_SECONDS)
                .then(|| {
                    (
                        (priority.priority)(*position, focus.center),
                        request.order,
                        *position,
                    )
                })
        })
        .collect::<Vec<_>>();
    if ready.len() > MAX_REQUESTS_PER_FRAME {
        ready.select_nth_unstable(MAX_REQUESTS_PER_FRAME);
        ready.truncate(MAX_REQUESTS_PER_FRAME);
    }
    ready.sort_unstable();

    for (_, _, position) in ready {
        let message = ServerBoundMessage::ChunkRequest(ChunkRequest { position });
        if let Err(error) = sender.send(&message) {
            warn!("failed to request chunk {position:?}: {error}");
        } else {
            if let Some(request) = pending.entries.get_mut(&position) {
                request.last_sent_at = Some(now);
            }
        }
    }
}

fn clear_pending_requests(mut pending: ResMut<PendingChunkRequests>) {
    pending.entries.clear();
    pending.next_order = 0;
}
