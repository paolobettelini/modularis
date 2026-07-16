use bevy_mod::BevyMod;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeFeatureContext, BiomeFeatureId, BiomeFeaturePhase, FeatureVerticalRange, ServerBiomeApi,
    ServerBiomeFeature, ServerBiomeRegistry,
};
use tokio::task::JoinHandle;
use voxel_math_api::{BlockPos, CHUNK_SIZE};

pub const CACTI_FEATURE_ID: &str = "demo:cacti";

pub fn cacti_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new(CACTI_FEATURE_ID)
}

pub struct ServerBiomeFeatureCactiVanillaMod;

impl ServerBiomeFeatureCactiVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _cactus: &mut block_cactus::BlockCactusMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_feature(cacti_feature_id(), CactiFeature)
            .expect("the vanilla cacti feature id must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

struct CactiFeature;

impl ServerBiomeFeature for CactiFeature {
    fn phase(&self) -> BiomeFeaturePhase {
        BiomeFeaturePhase::Decoration
    }

    fn vertical_range(&self) -> FeatureVerticalRange {
        FeatureVerticalRange::RelativeToSurface { min: 1, max: 4 }
    }

    fn generate(&self, context: &mut BiomeFeatureContext<'_>) {
        let origin = context.chunk_position().world_origin();
        for z in origin.z..(origin.z + CHUNK_SIZE) {
            for x in origin.x..(origin.x + CHUNK_SIZE) {
                if !context.is_target_biome(x, z)
                    || (x as i64 * x as i64 + z as i64 * z as i64) < 100
                {
                    continue;
                }
                let surface = context.surface_height(x, z);
                let anchor = BlockPos::new(x, surface + 1, z);
                let hash = context.hash(anchor, 0x4341_4354_5553);
                if hash % 67 != 0 {
                    continue;
                }
                let height = 2 + (hash.rotate_left(9) % 3) as i32;
                for y in 0..height {
                    context.set_block(BlockPos::new(x, surface + 1 + y, z), BlockId::Cactus);
                }
            }
        }
    }
}
