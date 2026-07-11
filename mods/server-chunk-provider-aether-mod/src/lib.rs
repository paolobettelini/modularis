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

pub const AETHER_PROVIDER_ID: &str = "demo:aether-islands";
const AETHER_SEED: u32 = 0x4145_5448;

pub struct ServerChunkProviderAetherMod;

impl ServerChunkProviderAetherMod {
    pub fn init<B: BlockManagerApi>(
        bevy: &mut BevyMod,
        _registry: &mut ServerChunkProviderRegistryMod,
        _blocks: &mut B,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerChunkProviderRegistry>()
            .register(
                ChunkProviderId::new(AETHER_PROVIDER_ID),
                AetherIslandProvider,
            )
            .expect("the Aether chunk provider id must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

struct AetherIslandProvider;

impl ServerChunkProvider for AetherIslandProvider {
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
                chunk.set(local, aether_block(local.to_world(position)));
            }
        }
    }
    chunk
}

fn aether_block(position: BlockPos) -> BlockId {
    let Some((surface, thickness)) = island_column(position.x, position.z) else {
        return BlockId::Air;
    };
    let depth = surface - position.y;
    match depth {
        i32::MIN..=-1 => BlockId::Air,
        0 => BlockId::Grass,
        1..=2 => BlockId::Dirt,
        depth if depth <= thickness => {
            if aether_hash(position) % 113 == 0 {
                BlockId::Glowstone
            } else {
                BlockId::Stone
            }
        }
        _ => BlockId::Air,
    }
}

fn island_column(x: i32, z: i32) -> Option<(i32, i32)> {
    let distance = ((x as f32).powi(2) + (z as f32).powi(2)).sqrt();
    if distance <= 8.0 {
        return Some((9, 4));
    }
    let noise = PerlinNoise2d::new(AETHER_SEED);
    let continents = noise.sample(x as f32 * 0.035, z as f32 * 0.035);
    let breakup = noise.sample(x as f32 * 0.085 + 31.0, z as f32 * 0.085 - 17.0);
    if continents + breakup * 0.38 < 0.12 {
        return None;
    }
    let height = noise.sample(x as f32 * 0.055 - 8.0, z as f32 * 0.055 + 12.0);
    let thickness = noise.sample(x as f32 * 0.071 + 19.0, z as f32 * 0.071 + 3.0);
    Some((
        (9.0 + height * 2.0).round() as i32,
        (3.5 + thickness * 1.5).round() as i32,
    ))
}

fn aether_hash(position: BlockPos) -> u32 {
    let mut value = AETHER_SEED
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
    fn spawn_island_has_air_below_and_safe_grass_above() {
        assert_eq!(aether_block(BlockPos::new(0, 9, 0)), BlockId::Grass);
        assert_eq!(aether_block(BlockPos::new(0, 10, 0)), BlockId::Air);
        assert_eq!(aether_block(BlockPos::new(0, 4, 0)), BlockId::Air);
    }

    #[test]
    fn distant_columns_include_air_gaps() {
        assert!(
            (-256..=256)
                .step_by(8)
                .any(|x| island_column(x, 160).is_none())
        );
    }
}
