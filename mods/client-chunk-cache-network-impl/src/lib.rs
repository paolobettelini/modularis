use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_edit_events_api::{BlockBroken, BlockPlaced};
use block_edit_events_mod::BlockEditEventsMod;
use client_chunk_cache_api::{
    ClientChunkAvailable, ClientChunkCache, ClientChunkCacheApi, ClientChunkChanged,
};
use client_chunk_streaming_api::ChunkUnload;
use client_game_state_api::{GameState, GameStateApi};
use generated_network_messages::{ChunkResponseReceived, NetworkMessageSet};
use network_protocol_mod::NetworkProtocolMod;
use tokio::task::JoinHandle;

pub struct ClientNetworkChunkCache;

impl ClientNetworkChunkCache {
    pub fn init<G: GameStateApi>(
        bevy: &mut BevyMod,
        _protocol: &mut NetworkProtocolMod,
        _block_edits: &mut BlockEditEventsMod,
        _streaming: &mut impl client_chunk_streaming_api::ChunkStreamingApi,
        _game_state: &mut G,
        _session: &mut impl client_session_api::ClientSessionApi,
    ) -> Self {
        bevy.app
            .init_resource::<ClientChunkCache>()
            .add_message::<ClientChunkAvailable>()
            .add_message::<ClientChunkChanged>()
            .add_systems(OnExit(GameState::InGame), clear_disconnected_cache)
            .add_systems(
                Update,
                (cache_chunks, apply_block_edits, remove_unloaded_chunks)
                    .chain()
                    .after(NetworkMessageSet::DispatchPackets)
                    .after(client_voxel_frame_api::ClientVoxelFrameSet::Receive)
                    .after(client_dimension_api::ClientDimensionSet::ResetWorld)
                    .after(client_world_context_api::ClientWorldContextSet::ResetWorld),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn clear_disconnected_cache(cache: Res<ClientChunkCache>) {
    cache.clear();
}

impl ClientChunkCacheApi for ClientNetworkChunkCache {}

fn cache_chunks(
    session: Res<client_session_api::ClientSession>,
    cache: Res<ClientChunkCache>,
    mut responses: MessageReader<ChunkResponseReceived>,
    mut available: MessageWriter<ClientChunkAvailable>,
) {
    for response in responses.read() {
        if response.0.movement_epoch != session.movement_epoch { continue; }
        let position = voxel_frame_api::VoxelChunkAddress::new(response.0.frame,response.0.chunk.position());
        cache.insert_in_frame(response.0.frame,response.0.chunk.clone());
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
        let was_empty = cache
            .uniform_block(unload.position)
            .is_some_and(|block| block.block == generated_block_registry::BlockId::Air);
        cache.remove(unload.position);
        if was_empty {
            continue;
        }
        for position in neighboring_chunk_positions(unload.position) {
            changed.write(ClientChunkChanged { position });
        }
    }
}

fn neighboring_chunk_positions(
    address: voxel_frame_api::VoxelChunkAddress,
) -> [voxel_frame_api::VoxelChunkAddress; 6] {
    let position = address.local;
    [
        voxel_math_api::ChunkPos::new(position.x + 1, position.y, position.z),
        voxel_math_api::ChunkPos::new(position.x - 1, position.y, position.z),
        voxel_math_api::ChunkPos::new(position.x, position.y + 1, position.z),
        voxel_math_api::ChunkPos::new(position.x, position.y - 1, position.z),
        voxel_math_api::ChunkPos::new(position.x, position.y, position.z + 1),
        voxel_math_api::ChunkPos::new(position.x, position.y, position.z - 1),
    ].map(|local| voxel_frame_api::VoxelChunkAddress::new(address.frame,local))
}
