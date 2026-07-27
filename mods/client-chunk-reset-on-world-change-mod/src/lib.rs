use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_chunk_cache_api::{ClientChunkCache, ClientChunkCacheApi};
use client_chunk_streaming_api::{ActiveChunks, ChunkStreamingApi, ChunkUnload};
use client_world_context_api::{ClientWorldChanged, ClientWorldContextApi, ClientWorldContextSet};
use tokio::task::JoinHandle;

pub struct ClientChunkResetOnWorldChangeMod;

impl ClientChunkResetOnWorldChangeMod {
    pub fn init<W: ClientWorldContextApi, C: ClientChunkCacheApi, S: ChunkStreamingApi>(
        bevy: &mut BevyMod,
        _world: &mut W,
        _cache: &mut C,
        _streaming: &mut S,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            reset_chunks.in_set(ClientWorldContextSet::ResetWorld),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn reset_chunks(
    cache: Res<ClientChunkCache>,
    mut active: ResMut<ActiveChunks>,
    mut changes: MessageReader<ClientWorldChanged>,
    mut unloads: MessageWriter<ChunkUnload>,
) {
    if changes.read().next().is_none() {
        return;
    }
    cache.clear();
    for position in active.positions.drain() {
        unloads.write(ChunkUnload { position });
    }
}
