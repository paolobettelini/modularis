use bevy_mod::BevyMod;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeFeatureContext, BiomeFeatureId, BiomeFeaturePhase, FeatureVerticalRange, ServerBiomeApi,
    ServerBiomeFeature, ServerBiomeRegistry,
};
use tokio::task::JoinHandle;
use voxel_math_api::{BlockPos, CHUNK_SIZE};

pub const SPARSE_SHORT_GRASS_FEATURE_ID: &str = "demo:short-grass-sparse";
pub const DENSE_SHORT_GRASS_FEATURE_ID: &str = "demo:short-grass-dense";

pub fn sparse_short_grass_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new(SPARSE_SHORT_GRASS_FEATURE_ID)
}

pub fn dense_short_grass_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new(DENSE_SHORT_GRASS_FEATURE_ID)
}

pub struct ServerBiomeFeatureShortGrassVanillaMod;

impl ServerBiomeFeatureShortGrassVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _grass: &mut block_short_grass::BlockShortGrassMod,
    ) -> Self {
        let registry = bevy.app.world().resource::<ServerBiomeRegistry>();
        registry
            .register_feature(
                sparse_short_grass_feature_id(),
                ShortGrassFeature {
                    chance_denominator: 4,
                },
            )
            .expect("the sparse short-grass feature id must be unique");
        registry
            .register_feature(
                dense_short_grass_feature_id(),
                ShortGrassFeature {
                    chance_denominator: 2,
                },
            )
            .expect("the dense short-grass feature id must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

struct ShortGrassFeature {
    chance_denominator: u64,
}

impl ServerBiomeFeature for ShortGrassFeature {
    fn phase(&self) -> BiomeFeaturePhase {
        BiomeFeaturePhase::Decoration
    }

    fn vertical_range(&self) -> FeatureVerticalRange {
        FeatureVerticalRange::RelativeToSurface { min: 1, max: 1 }
    }

    fn generate(&self, context: &mut BiomeFeatureContext<'_>) {
        let origin = context.chunk_position().world_origin();
        for z in origin.z..(origin.z + CHUNK_SIZE) {
            for x in origin.x..(origin.x + CHUNK_SIZE) {
                if !context.is_target_biome(x, z) {
                    continue;
                }
                let position = BlockPos::new(x, context.surface_height(x, z) + 1, z);
                if context.hash(position, 0x5348_4f52_5447_5253) % self.chance_denominator != 0 {
                    continue;
                }
                if context
                    .block(position)
                    .is_some_and(|block| block.block == BlockId::Air)
                {
                    context.set_block(position, BlockId::ShortGrass);
                }
            }
        }
    }
}
