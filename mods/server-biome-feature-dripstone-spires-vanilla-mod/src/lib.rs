use bevy_mod::BevyMod;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeFeatureContext, BiomeFeatureId, BiomeFeaturePhase, FeatureVerticalRange, ServerBiomeApi,
    ServerBiomeFeature, ServerBiomeRegistry,
};
use tokio::task::JoinHandle;
use voxel_math_api::{BlockPos, CHUNK_SIZE};
pub fn dripstone_spires_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new("demo:dripstone-spires")
}
pub struct ServerBiomeFeatureDripstoneSpiresVanillaMod;
impl ServerBiomeFeatureDripstoneSpiresVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _dripstone_block: &mut block_dripstone_block::BlockDripstoneBlockMod,
        _tuff: &mut block_tuff::BlockTuffMod,
        _cobbled_deepslate: &mut block_cobbled_deepslate::BlockCobbledDeepslateMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_feature(dripstone_spires_feature_id(), DripstoneSpiresFeature)
            .expect("dripstone spire feature id must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
struct DripstoneSpiresFeature;
impl ServerBiomeFeature for DripstoneSpiresFeature {
    fn phase(&self) -> BiomeFeaturePhase {
        BiomeFeaturePhase::Decoration
    }
    fn vertical_range(&self) -> FeatureVerticalRange {
        FeatureVerticalRange::RelativeToSurface { min: 0, max: 9 }
    }
    fn generate(&self, c: &mut BiomeFeatureContext<'_>) {
        let o = c.chunk_position().world_origin();
        for z in (o.z - 1)..=o.z + CHUNK_SIZE {
            for x in (o.x - 1)..=o.x + CHUNK_SIZE {
                if !c.is_target_biome(x, z) {
                    continue;
                }
                let s = c.surface_height(x, z);
                let h = c.hash(BlockPos::new(x, s, z), 0x4452_4950_5350_4952);
                if h % 47 != 0 {
                    continue;
                }
                let height = 2 + (h.rotate_left(7) % 6) as i32;
                c.set_block(
                    BlockPos::new(x, s, z),
                    if h % 4 == 0 {
                        BlockId::CobbledDeepslate
                    } else {
                        BlockId::Tuff
                    },
                );
                for y in 1..=height {
                    c.set_block(BlockPos::new(x, s + y, z), BlockId::DripstoneBlock);
                }
            }
        }
    }
}
