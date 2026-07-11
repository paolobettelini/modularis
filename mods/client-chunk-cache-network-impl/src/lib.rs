use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_edit_events_api::{BlockBroken, BlockPlaced};
use block_edit_events_mod::BlockEditEventsMod;
use client_chunk_cache_api::{
    ClientChunkAvailable, ClientChunkCache, ClientChunkCacheApi, ClientChunkChanged,
};
use client_chunk_streaming_api::ChunkUnload;
use generated_network_messages::{ChunkResponseReceived, NetworkMessageSet};
use network_protocol_mod::NetworkProtocolMod;
use tokio::task::JoinHandle;

pub struct ClientNetworkChunkCache;

impl ClientNetworkChunkCache {
    pub fn init(
        bevy: &mut BevyMod,
        _protocol: &mut NetworkProtocolMod,
        _block_edits: &mut BlockEditEventsMod,
        _streaming: &mut impl client_chunk_streaming_api::ChunkStreamingApi,
    ) -> Self {
        bevy.app
            .init_resource::<ClientChunkCache>()
            .add_message::<ClientChunkAvailable>()
            .add_message::<ClientChunkChanged>()
            .add_systems(
                Update,
                (cache_chunks, apply_block_edits, remove_unloaded_chunks)
                    .chain()
                    .after(NetworkMessageSet::DispatchPackets),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ClientChunkCacheApi for ClientNetworkChunkCache {}

fn cache_chunks(
    cache: Res<ClientChunkCache>,
    mut responses: MessageReader<ChunkResponseReceived>,
    mut available: MessageWriter<ClientChunkAvailable>,
) {
    for response in responses.read() {
        let position = response.0.chunk.position();
        cache.insert(response.0.chunk.clone());
        available.write(ClientChunkAvailable { position });
    }
}

fn apply_block_edits(
    cache: Res<ClientChunkCache>,
    mut broken: MessageReader<BlockBroken>,
    mut placed: MessageReader<BlockPlaced>,
    mut changed: MessageWriter<ClientChunkChanged>,
) {
    for event in broken.read() {
        if cache.set_block(event.position, generated_block_registry::BlockId::Air) {
            changed.write(ClientChunkChanged {
                position: event.position.chunk(),
            });
        }
    }
    for event in placed.read() {
        if cache.set_block(event.position, event.block.clone()) {
            changed.write(ClientChunkChanged {
                position: event.position.chunk(),
            });
        }
    }
}

fn remove_unloaded_chunks(
    cache: Res<ClientChunkCache>,
    mut unloads: MessageReader<ChunkUnload>,
    mut changed: MessageWriter<ClientChunkChanged>,
) {
    for unload in unloads.read() {
        cache.remove(unload.position);
        for position in neighboring_chunk_positions(unload.position) {
            changed.write(ClientChunkChanged { position });
        }
    }
}

fn neighboring_chunk_positions(
    position: voxel_math_api::ChunkPos,
) -> [voxel_math_api::ChunkPos; 6] {
    [
        voxel_math_api::ChunkPos::new(position.x + 1, position.y, position.z),
        voxel_math_api::ChunkPos::new(position.x - 1, position.y, position.z),
        voxel_math_api::ChunkPos::new(position.x, position.y + 1, position.z),
        voxel_math_api::ChunkPos::new(position.x, position.y - 1, position.z),
        voxel_math_api::ChunkPos::new(position.x, position.y, position.z + 1),
        voxel_math_api::ChunkPos::new(position.x, position.y, position.z - 1),
    ]
}
