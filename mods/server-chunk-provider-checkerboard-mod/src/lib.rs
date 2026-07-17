use bevy_mod::BevyMod;
use block_manager_api::BlockManagerApi;
use chunk_api::Chunk;
use generated_block_registry::BlockId;
use server_chunk_provider_api::{
    ChunkGenerationRequest, ChunkProviderId, ServerChunkProvider, ServerChunkProviderRegistry,
};
use server_chunk_provider_registry_mod::ServerChunkProviderRegistryMod;
use server_primary_chunk_provider_api::ServerPrimaryChunkProviderApi;
use server_world_seed_api::{ServerWorldSeed, ServerWorldSeedApi};
use tokio::task::JoinHandle;
use voxel_math_api::{BlockPos, CHUNK_SIZE, LocalBlockPos};

pub struct ServerChunkProviderCheckerboardMod;

const CHECKERBOARD_SEED_NAMESPACE: &str = "demo:checkerboard-terrain";

impl ServerChunkProviderCheckerboardMod {
    pub fn init<B: BlockManagerApi, W: ServerWorldSeedApi>(
        bevy: &mut BevyMod,
        _registry_mod: &mut ServerChunkProviderRegistryMod,
        _blocks: &mut B,
        _world_seed: &mut W,
    ) -> Self {
        let seed = *bevy.app.world().resource::<ServerWorldSeed>();
        bevy.app
            .world()
            .resource::<ServerChunkProviderRegistry>()
            .register(
                ChunkProviderId::primary(),
                CheckerboardChunkProvider { seed },
            )
            .expect("the primary server chunk provider must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerPrimaryChunkProviderApi for ServerChunkProviderCheckerboardMod {}

struct CheckerboardChunkProvider {
    seed: ServerWorldSeed,
}

impl ServerChunkProvider for CheckerboardChunkProvider {
    fn generate(&self, request: &ChunkGenerationRequest) -> Option<Chunk> {
        Some(build_chunk(
            request.position,
            self.seed
                .derive(CHECKERBOARD_SEED_NAMESPACE, &request.instance),
        ))
    }
}

fn build_chunk(position: voxel_math_api::ChunkPos, seed: u64) -> Chunk {
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
                chunk.set(local, base_block(local.to_world(position), seed));
            }
        }
    }
    chunk
}

fn base_block(position: BlockPos, seed: u64) -> BlockId {
    match position.y {
        i32::MIN..=-1 => BlockId::Stone,
        0 => BlockId::Bedrock,
        1 if (position.x + position.z + seed as i32).rem_euclid(2) == 0 => BlockId::Dirt,
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
        let first = build_chunk(position, 42);
        let second = build_chunk(position, 42);
        assert_eq!(first, second);
        assert_eq!(
            first.get(LocalBlockPos::new(0, 0, 0).unwrap()).block,
            BlockId::Bedrock
        );
    }
}
