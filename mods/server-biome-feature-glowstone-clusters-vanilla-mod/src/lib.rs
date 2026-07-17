use bevy_mod::BevyMod;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeFeatureContext, BiomeFeatureId, BiomeFeaturePhase, FeatureVerticalRange, ServerBiomeApi,
    ServerBiomeFeature, ServerBiomeRegistry,
};
use tokio::task::JoinHandle;
use voxel_math_api::{BlockPos, CHUNK_SIZE};

pub const GLOWSTONE_CLUSTERS_FEATURE_ID: &str = "demo:glowstone-clusters";

pub fn glowstone_clusters_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new(GLOWSTONE_CLUSTERS_FEATURE_ID)
}

pub struct ServerBiomeFeatureGlowstoneClustersVanillaMod;

impl ServerBiomeFeatureGlowstoneClustersVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _glowstone: &mut block_glowstone::BlockGlowstoneMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_feature(glowstone_clusters_feature_id(), GlowstoneClustersFeature)
            .expect("the glowstone cluster feature id must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

struct GlowstoneClustersFeature;

impl ServerBiomeFeature for GlowstoneClustersFeature {
    fn phase(&self) -> BiomeFeaturePhase {
        BiomeFeaturePhase::Decoration
    }

    fn vertical_range(&self) -> FeatureVerticalRange {
        FeatureVerticalRange::RelativeToSurface { min: 1, max: 5 }
    }

    fn generate(&self, context: &mut BiomeFeatureContext<'_>) {
        let origin = context.chunk_position().world_origin();
        for z in (origin.z - 1)..=(origin.z + CHUNK_SIZE) {
            for x in (origin.x - 1)..=(origin.x + CHUNK_SIZE) {
                if !context.is_target_biome(x, z) {
                    continue;
                }
                let surface = context.surface_height(x, z);
                let anchor = BlockPos::new(x, surface + 1, z);
                let hash = context.hash(anchor, 0x474c_4f57_5354_4f4e);
                if hash % 97 != 0 {
                    continue;
                }
                let height = 1 + (hash.rotate_left(9) % 3) as i32;
                for y in 1..=height {
                    let position = BlockPos::new(x, surface + y, z);
                    if context
                        .block(position)
                        .is_some_and(|block| block.block == BlockId::Air)
                    {
                        context.set_block(position, BlockId::Glowstone);
                    }
                }
            }
        }
    }
}
