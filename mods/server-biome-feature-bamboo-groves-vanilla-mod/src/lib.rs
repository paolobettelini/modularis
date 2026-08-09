use bevy_mod::BevyMod;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeFeatureContext, BiomeFeatureId, BiomeFeaturePhase, FeatureVerticalRange, ServerBiomeApi,
    ServerBiomeFeature, ServerBiomeRegistry,
};
use tokio::task::JoinHandle;
use voxel_math_api::{BlockPos, CHUNK_SIZE};
pub fn bamboo_groves_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new("demo:bamboo-groves")
}
pub struct ServerBiomeFeatureBambooGrovesVanillaMod;
impl ServerBiomeFeatureBambooGrovesVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _bamboo_block: &mut block_bamboo_block::BlockBambooBlockMod,
        _stripped_bamboo_block: &mut block_stripped_bamboo_block::BlockStrippedBambooBlockMod,
        _bamboo_mosaic: &mut block_bamboo_mosaic::BlockBambooMosaicMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_feature(bamboo_groves_feature_id(), BambooGrovesFeature)
            .expect("bamboo grove feature id must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
struct BambooGrovesFeature;
impl ServerBiomeFeature for BambooGrovesFeature {
    fn phase(&self) -> BiomeFeaturePhase {
        BiomeFeaturePhase::Decoration
    }
    fn vertical_range(&self) -> FeatureVerticalRange {
        FeatureVerticalRange::RelativeToSurface { min: 0, max: 10 }
    }
    fn generate(&self, c: &mut BiomeFeatureContext<'_>) {
        let o = c.chunk_position().world_origin();
        for z in (o.z - 1)..=o.z + CHUNK_SIZE {
            for x in (o.x - 1)..=o.x + CHUNK_SIZE {
                if !c.is_target_biome(x, z) {
                    continue;
                }
                let y = c.surface_height(x, z);
                let a = BlockPos::new(x, y + 1, z);
                let h = c.hash(a, 0x4241_4d42_4f4f_4752);
                if h % 17 > 3 {
                    continue;
                }
                let height = 4 + (h.rotate_left(9) % 6) as i32;
                for dy in 1..=height {
                    c.set_block(
                        BlockPos::new(x, y + dy, z),
                        if dy == 1 && h % 7 == 0 {
                            BlockId::StrippedBambooBlock
                        } else {
                            BlockId::BambooBlock
                        },
                    );
                }
                if h % 19 == 0 {
                    c.set_block(BlockPos::new(x, y, z), BlockId::BambooMosaic);
                }
            }
        }
    }
}
