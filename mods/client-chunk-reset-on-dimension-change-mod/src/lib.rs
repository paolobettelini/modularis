use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_chunk_cache_api::{ClientChunkCache, ClientChunkCacheApi};
use client_chunk_streaming_api::{ActiveChunks, ChunkStreamingApi, ChunkUnload};
use client_dimension_api::{ClientDimensionApi, ClientDimensionChanged, ClientDimensionSet};
use tokio::task::JoinHandle;

pub struct ClientChunkResetOnDimensionChangeMod;

impl ClientChunkResetOnDimensionChangeMod {
    pub fn init<D: ClientDimensionApi, C: ClientChunkCacheApi, S: ChunkStreamingApi>(
        bevy: &mut BevyMod,
        _dimension: &mut D,
        _cache: &mut C,
        _streaming: &mut S,
    ) -> Self {
        bevy.app
            .add_systems(Update, reset_chunks.in_set(ClientDimensionSet::ResetWorld));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn reset_chunks(
    cache: Res<ClientChunkCache>,
    mut active: ResMut<ActiveChunks>,
    mut changes: MessageReader<ClientDimensionChanged>,
    mut unloads: MessageWriter<ChunkUnload>,
) {
    if !changes
        .read()
        .any(|change| change.previous != change.current)
    {
        return;
    }
    cache.clear();
    for position in active.positions.drain() {
        unloads.write(ChunkUnload { position });
    }
}
