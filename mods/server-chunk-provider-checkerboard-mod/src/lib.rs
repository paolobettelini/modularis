use bevy_mod::BevyMod;
use block_manager_api::BlockManagerApi;
use chunk_api::Chunk;
use generated_block_registry::BlockId;
use server_chunk_provider_api::{
    ChunkGenerationRequest, ChunkProviderId, ServerChunkProvider, ServerChunkProviderRegistry,
};
use server_chunk_provider_registry_mod::ServerChunkProviderRegistryMod;
use server_primary_chunk_provider_api::ServerPrimaryChunkProviderApi;
use tokio::task::JoinHandle;
use voxel_math_api::{BlockPos, CHUNK_SIZE, LocalBlockPos};

pub struct ServerChunkProviderCheckerboardMod;

impl ServerChunkProviderCheckerboardMod {
    pub fn init<B: BlockManagerApi>(
        bevy: &mut BevyMod,
        _registry_mod: &mut ServerChunkProviderRegistryMod,
        _blocks: &mut B,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerChunkProviderRegistry>()
            .register(ChunkProviderId::primary(), CheckerboardChunkProvider)
            .expect("the primary server chunk provider must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerPrimaryChunkProviderApi for ServerChunkProviderCheckerboardMod {}

struct CheckerboardChunkProvider;

impl ServerChunkProvider for CheckerboardChunkProvider {
    fn generate(&self, request: &ChunkGenerationRequest) -> Option<Chunk> {
        Some(build_chunk(request.position))
    }
}

fn build_chunk(position: voxel_math_api::ChunkPos) -> Chunk {
    if position.y > 0 {
        return Chunk::filled(position, BlockId::Air);
    }
    if position.y < 0 {
        return Chunk::filled(position, BlockId::Stone);
    }
    let mut chunk = Chunk::filled(position, BlockId::Air);
    for local_y in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let local = LocalBlockPos::new(x, local_y, z).unwrap();
                chunk.set(local, base_block(local.to_world(position)));
            }
        }
    }
    chunk
}

fn base_block(position: BlockPos) -> BlockId {
    match position.y {
        i32::MIN..=-1 => BlockId::Stone,
        0 => BlockId::Bedrock,
        1 if (position.x + position.z).rem_euclid(2) == 0 => BlockId::Dirt,
        1 => BlockId::Stone,
        _ => BlockId::Air,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checkerboard_generation_is_unbounded_and_deterministic() {
        let position = voxel_math_api::ChunkPos::new(10_000, 0, -10_000);
        let first = build_chunk(position);
        let second = build_chunk(position);
        assert_eq!(first, second);
        assert_eq!(
            first.get(LocalBlockPos::new(0, 0, 0).unwrap()).block,
            BlockId::Bedrock
        );
    }
}
