use bevy_mod::BevyMod;
use coherent_noise_api::PerlinNoise2d;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeFeatureContext, BiomeFeatureId, BiomeFeaturePhase, FeatureVerticalRange, ServerBiomeApi,
    ServerBiomeFeature, ServerBiomeRegistry,
};
use tokio::task::JoinHandle;
use voxel_math_api::{CHUNK_SIZE, LocalBlockPos};

pub const CAVES_FEATURE_ID: &str = "demo:caves";

pub fn caves_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new(CAVES_FEATURE_ID)
}

pub struct ServerBiomeFeatureCavesVanillaMod;

impl ServerBiomeFeatureCavesVanillaMod {
    pub fn init<B: ServerBiomeApi>(bevy: &mut BevyMod, _biomes: &mut B) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_feature(caves_feature_id(), CavesFeature)
            .expect("the vanilla cave feature id must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

struct CavesFeature;

impl ServerBiomeFeature for CavesFeature {
    fn phase(&self) -> BiomeFeaturePhase {
        BiomeFeaturePhase::Carving
    }

    fn vertical_range(&self) -> FeatureVerticalRange {
        FeatureVerticalRange::Absolute { min: -48, max: 64 }
    }

    fn generate(&self, context: &mut BiomeFeatureContext<'_>) {
        let seed = (context.world_seed() as u32) ^ 0x4341_5645;
        let horizontal = PerlinNoise2d::new(seed);
        let vertical = PerlinNoise2d::new(seed ^ 0x9e37_79b9);
        let chunk = context.chunk_position();
        for local_y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let local = LocalBlockPos::new(x, local_y, z).unwrap();
                    let world = local.to_world(chunk);
                    if !context.is_target_biome(world.x, world.z)
                        || world.y < -48
                        || world.y >= context.surface_height(world.x, world.z) - 3
                    {
                        continue;
                    }
                    let a = horizontal.sample(
                        world.x as f32 * 0.052 + world.y as f32 * 0.018,
                        world.z as f32 * 0.052,
                    );
                    let b = vertical.sample(
                        world.x as f32 * 0.041,
                        world.z as f32 * 0.021 + world.y as f32 * 0.047,
                    );
                    let tunnel = a * 0.62 + b * 0.38;
                    if tunnel.abs() < 0.045
                        && context.block(world).is_some_and(|block| {
                            block.block != BlockId::Air && block.block != BlockId::Bedrock
                        })
                    {
                        context.set_block(world, BlockId::Air);
                    }
                }
            }
        }
    }
}
