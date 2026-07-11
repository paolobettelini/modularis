use bevy_mod::BevyMod;
use block_manager_api::BlockManagerApi;
use chunk_api::Chunk;
use coherent_noise_api::PerlinNoise2d;
use generated_block_registry::BlockId;
use server_chunk_provider_api::{
    ChunkGenerationRequest, ChunkProviderId, ServerChunkProvider, ServerChunkProviderRegistry,
};
use server_chunk_provider_registry_mod::ServerChunkProviderRegistryMod;
use tokio::task::JoinHandle;
use voxel_math_api::{BlockPos, CHUNK_SIZE, LocalBlockPos};

pub const NETHER_PROVIDER_ID: &str = "demo:nether-terrain";
const NETHER_SEED: u32 = 0x4e45_5448;

pub struct ServerChunkProviderNetherMod;

impl ServerChunkProviderNetherMod {
    pub fn init<B: BlockManagerApi>(
        bevy: &mut BevyMod,
        _registry_mod: &mut ServerChunkProviderRegistryMod,
        _blocks: &mut B,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerChunkProviderRegistry>()
            .register(
                ChunkProviderId::new(NETHER_PROVIDER_ID),
                NetherTerrainProvider,
            )
            .expect("the Nether chunk provider id must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

struct NetherTerrainProvider;

impl ServerChunkProvider for NetherTerrainProvider {
    fn generate(&self, request: &ChunkGenerationRequest) -> Option<Chunk> {
        Some(build_chunk(request.position))
    }
}

fn build_chunk(position: voxel_math_api::ChunkPos) -> Chunk {
    let mut chunk = Chunk::filled(position, BlockId::Air);
    for local_y in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let local = LocalBlockPos::new(x, local_y, z).unwrap();
                chunk.set(local, nether_block(local.to_world(position)));
            }
        }
    }
    chunk
}

fn nether_block(position: BlockPos) -> BlockId {
    let surface = nether_height(position.x, position.z);
    match position.y {
        i32::MIN..=-1 => BlockId::Netherrack,
        0 => BlockId::Bedrock,
        y if y > surface => BlockId::Air,
        _ if obsidian_hash(position) % 127 == 0 => BlockId::Obsidian,
        _ => BlockId::Netherrack,
    }
}

fn nether_height(x: i32, z: i32) -> i32 {
    let noise = PerlinNoise2d::new(NETHER_SEED);
    let large = noise.sample(x as f32 * 0.04, z as f32 * 0.04);
    let detail = noise.sample(x as f32 * 0.11 - 22.0, z as f32 * 0.11 + 9.0);
    let distant = (4.0 + large * 2.5 + detail).clamp(1.0, 8.0);
    let distance = ((x as f32).powi(2) + (z as f32).powi(2)).sqrt();
    let blend = (((distance - 8.0) / 40.0).clamp(0.0, 1.0)).powi(2);
    (1.0 + (distant - 1.0) * blend).round() as i32
}

fn obsidian_hash(position: BlockPos) -> u32 {
    let mut value = NETHER_SEED
        ^ (position.x as u32).wrapping_mul(0x9e37_79b9)
        ^ (position.y as u32).wrapping_mul(0x85eb_ca6b)
        ^ (position.z as u32).wrapping_mul(0xc2b2_ae35);
    value ^= value >> 16;
    value = value.wrapping_mul(0x7feb_352d);
    value ^ (value >> 15)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nether_spawn_is_safe_and_terrain_varies() {
        assert_eq!(nether_height(0, 0), 1);
        let heights = (-128..=128)
            .step_by(8)
            .map(|x| nether_height(x, 80))
            .collect::<std::collections::HashSet<_>>();
        assert!(heights.len() >= 2);
    }
}
