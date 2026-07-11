use bevy_mod::BevyMod;
use block_manager_api::BlockManagerApi;
use chunk_api::Chunk;
use coherent_noise_api::PerlinNoise2d;
use generated_block_registry::BlockId;
use server_chunk_provider_api::{
    ChunkGenerationRequest, ChunkProviderId, ServerChunkProvider, ServerChunkProviderRegistry,
};
use server_chunk_provider_registry_mod::ServerChunkProviderRegistryMod;
use server_primary_chunk_provider_api::ServerPrimaryChunkProviderApi;
use tokio::task::JoinHandle;
use voxel_math_api::{BlockPos, CHUNK_SIZE, LocalBlockPos};

const TERRAIN_SEED: u32 = 0x5041_5443;

pub struct ServerChunkProviderPerlinMod;

impl ServerChunkProviderPerlinMod {
    pub fn init<B: BlockManagerApi>(
        bevy: &mut BevyMod,
        _registry_mod: &mut ServerChunkProviderRegistryMod,
        _blocks: &mut B,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerChunkProviderRegistry>()
            .register(ChunkProviderId::primary(), PerlinTerrainProvider)
            .expect("the primary server chunk provider must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerPrimaryChunkProviderApi for ServerChunkProviderPerlinMod {}

struct PerlinTerrainProvider;

impl ServerChunkProvider for PerlinTerrainProvider {
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
                let world = local.to_world(position);
                chunk.set(local, terrain_block(world));
            }
        }
    }
    chunk
}

fn terrain_block(position: BlockPos) -> BlockId {
    let surface = terrain_height(position.x, position.z);
    match position.y {
        i32::MIN..=-1 => BlockId::Stone,
        0 => BlockId::Bedrock,
        y if y > surface => BlockId::Air,
        y if y == surface => BlockId::Grass,
        y if y >= surface - 2 => BlockId::Dirt,
        _ if ore_hash(position) % 89 == 0 => BlockId::DiamondOre,
        _ => BlockId::Stone,
    }
}

fn terrain_height(x: i32, z: i32) -> i32 {
    let noise = PerlinNoise2d::new(TERRAIN_SEED);
    let macro_noise = noise.sample(x as f32 * 0.035, z as f32 * 0.035);
    let detail_noise = noise.sample(x as f32 * 0.09 + 31.7, z as f32 * 0.09 - 18.2);
    let distant_height = (5.0 + macro_noise * 3.0 + detail_noise * 1.25).clamp(1.0, 10.0);

    // Keep the shared spawn safe at the old y=2 and blend smoothly into hills.
    let distance = ((x as f32).powi(2) + (z as f32).powi(2)).sqrt();
    let blend = smoothstep(((distance - 8.0) / 48.0).clamp(0.0, 1.0));
    (1.0 + (distant_height - 1.0) * blend).round() as i32
}

fn smoothstep(value: f32) -> f32 {
    value * value * (3.0 - 2.0 * value)
}

fn ore_hash(position: BlockPos) -> u32 {
    let mut value = TERRAIN_SEED
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
    fn spawn_is_safe_and_distant_terrain_has_hills() {
        assert_eq!(terrain_height(0, 0), 1);
        let heights = (-128..=128)
            .step_by(8)
            .map(|x| terrain_height(x, 96))
            .collect::<std::collections::HashSet<_>>();
        assert!(heights.len() >= 3);
    }

    #[test]
    fn neighboring_chunks_share_world_space_height_function() {
        let left = terrain_height(15, 40);
        let right = terrain_height(16, 40);
        assert!((left - right).abs() <= 2);
    }

    #[test]
    fn vertical_chunks_outside_the_surface_layer_use_uniform_fast_paths() {
        assert_eq!(
            build_chunk(voxel_math_api::ChunkPos::new(0, 50, 0))
                .uniform_block()
                .unwrap()
                .block,
            BlockId::Air
        );
        assert_eq!(
            build_chunk(voxel_math_api::ChunkPos::new(0, -50, 0))
                .uniform_block()
                .unwrap()
                .block,
            BlockId::Stone
        );
    }
}
