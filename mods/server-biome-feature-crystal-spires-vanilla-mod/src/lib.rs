use bevy_mod::BevyMod;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeFeatureContext, BiomeFeatureId, BiomeFeaturePhase, FeatureVerticalRange, ServerBiomeApi,
    ServerBiomeFeature, ServerBiomeRegistry,
};
use tokio::task::JoinHandle;
use voxel_math_api::{BlockPos, CHUNK_SIZE};

pub const CRYSTAL_SPIRES_FEATURE_ID: &str = "demo:crystal-spires";

pub fn crystal_spires_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new(CRYSTAL_SPIRES_FEATURE_ID)
}

pub struct ServerBiomeFeatureCrystalSpiresVanillaMod;

impl ServerBiomeFeatureCrystalSpiresVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _calcite: &mut block_calcite::BlockCalciteMod,
        _glowstone: &mut block_glowstone::BlockGlowstoneMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_feature(crystal_spires_feature_id(), CrystalSpiresFeature)
            .expect("the crystal spire feature id must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

struct CrystalSpiresFeature;

impl ServerBiomeFeature for CrystalSpiresFeature {
    fn phase(&self) -> BiomeFeaturePhase {
        BiomeFeaturePhase::Decoration
    }

    fn vertical_range(&self) -> FeatureVerticalRange {
        FeatureVerticalRange::RelativeToSurface { min: 1, max: 8 }
    }

    fn generate(&self, context: &mut BiomeFeatureContext<'_>) {
        let origin = context.chunk_position().world_origin();
        for z in origin.z..(origin.z + CHUNK_SIZE) {
            for x in origin.x..(origin.x + CHUNK_SIZE) {
                if !context.is_target_biome(x, z) {
                    continue;
                }
                let surface = context.surface_height(x, z);
                let anchor = BlockPos::new(x, surface + 1, z);
                let hash = context.hash(anchor, 0x4352_5953_5441_4c53);
                if hash % 83 != 0 {
                    continue;
                }
                let height = 3 + (hash.rotate_left(7) % 5) as i32;
                for y in 1..=height {
                    let block = if y == height && hash.rotate_left(21) % 3 == 0 {
                        BlockId::Glowstone
                    } else {
                        BlockId::Calcite
                    };
                    context.set_block(BlockPos::new(x, surface + y, z), block);
                }
            }
        }
    }
}
