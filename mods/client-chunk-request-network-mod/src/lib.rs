use bevy::prelude::*;
use bevy_mod::BevyMod;
use chunk_network_message_types::ChunkRequest;
use client_chunk_cache_api::{ClientChunkAvailable, ClientChunkCache, ClientChunkCacheApi};
use client_chunk_streaming_api::{ActiveChunks, ChunkNeeded, ChunkStreamingApi, ChunkUnload};
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
}

#[derive(Resource, Default)]
struct PendingChunkRequests(HashMap<ChunkPos, PendingChunkRequest>);

pub struct ClientChunkRequestNetworkMod;

impl ClientChunkRequestNetworkMod {
    pub fn init<
        N: ClientNetworkApi,
        S: ChunkStreamingApi,
        C: ClientChunkCacheApi,
        G: GameStateApi,
    >(
        bevy: &mut BevyMod,
        _network: &mut N,
        _streaming: &mut S,
        _cache: &mut C,
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
        pending
            .0
            .entry(needed.position)
            .or_insert(PendingChunkRequest { last_sent_at: None });
    }
}

fn finish_chunk_requests(
    mut available: MessageReader<ClientChunkAvailable>,
    mut pending: ResMut<PendingChunkRequests>,
) {
    for available in available.read() {
        pending.0.remove(&available.position);
    }
}

fn forget_unloaded_chunks(
    mut unloads: MessageReader<ChunkUnload>,
    mut pending: ResMut<PendingChunkRequests>,
) {
    for unload in unloads.read() {
        pending.0.remove(&unload.position);
    }
}

fn reconcile_active_chunks(
    active: Res<ActiveChunks>,
    cache: Res<ClientChunkCache>,
    mut pending: ResMut<PendingChunkRequests>,
) {
    pending
        .0
        .retain(|position, _| active.positions.contains(position));
    for position in active.positions.iter().copied() {
        if cache.chunk(position).is_none() {
            pending
                .0
                .entry(position)
                .or_insert(PendingChunkRequest { last_sent_at: None });
        }
    }
}

fn send_chunk_requests(
    time: Res<Time>,
    sender: Option<Res<ClientNetworkSender>>,
    mut pending: ResMut<PendingChunkRequests>,
) {
    let Some(sender) = sender else {
        return;
    };

    let now = time.elapsed_secs_f64();
    let mut sent = 0;
    for (position, request) in &mut pending.0 {
        let ready = request
            .last_sent_at
            .is_none_or(|last_sent| now - last_sent >= RETRY_AFTER_SECONDS);
        if !ready {
            continue;
        }

        let message = ServerBoundMessage::ChunkRequest(ChunkRequest {
            position: *position,
        });
        if let Err(error) = sender.send(&message) {
            warn!("failed to request chunk {position:?}: {error}");
        } else {
            request.last_sent_at = Some(now);
            sent += 1;
            if sent >= MAX_REQUESTS_PER_FRAME {
                break;
            }
        }
    }
}

fn clear_pending_requests(mut pending: ResMut<PendingChunkRequests>) {
    pending.0.clear();
}
