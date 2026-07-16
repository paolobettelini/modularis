use bevy_mod::BevyMod;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeFeatureContext, BiomeFeatureId, BiomeFeaturePhase, FeatureVerticalRange, ServerBiomeApi,
    ServerBiomeFeature, ServerBiomeRegistry,
};
use tokio::task::JoinHandle;
use voxel_math_api::{CHUNK_SIZE, LocalBlockPos};

pub const ORES_FEATURE_ID: &str = "demo:diamond-ores";

pub fn ores_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new(ORES_FEATURE_ID)
}

pub struct ServerBiomeFeatureOresVanillaMod;

impl ServerBiomeFeatureOresVanillaMod {
    pub fn init<B: ServerBiomeApi>(bevy: &mut BevyMod, _biomes: &mut B) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_feature(ores_feature_id(), OresFeature)
            .expect("the vanilla ore feature id must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

struct OresFeature;

impl ServerBiomeFeature for OresFeature {
    fn phase(&self) -> BiomeFeaturePhase {
        BiomeFeaturePhase::Underground
    }

    fn vertical_range(&self) -> FeatureVerticalRange {
        FeatureVerticalRange::Absolute { min: -48, max: 32 }
    }

    fn generate(&self, context: &mut BiomeFeatureContext<'_>) {
        let chunk = context.chunk_position();
        let underground = context.definition().terrain.underground;
        for local_y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let world = LocalBlockPos::new(x, local_y, z).unwrap().to_world(chunk);
                    if !context.is_target_biome(world.x, world.z)
                        || !(-48..=32).contains(&world.y)
                        || context
                            .block(world)
                            .is_none_or(|block| block.block != underground)
                    {
                        continue;
                    }
                    let rarity = if world.y <= 4 { 83 } else { 137 };
                    if context.hash(world, 0x4f52_4553) % rarity == 0 {
                        context.set_block(world, BlockId::DiamondOre);
                    }
                }
            }
        }
    }
}
