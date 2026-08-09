use bevy_mod::BevyMod;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeFeatureContext, BiomeFeatureId, BiomeFeaturePhase, FeatureVerticalRange, ServerBiomeApi,
    ServerBiomeFeature, ServerBiomeRegistry,
};
use tokio::task::JoinHandle;
use voxel_math_api::{BlockPos, CHUNK_SIZE};
pub fn melon_patches_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new("demo:melon-patches")
}
pub struct ServerBiomeFeatureMelonPatchesVanillaMod;
impl ServerBiomeFeatureMelonPatchesVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _melon: &mut block_melon::BlockMelonMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_feature(melon_patches_feature_id(), MelonPatchesFeature)
            .expect("melon patch feature id must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
struct MelonPatchesFeature;
impl ServerBiomeFeature for MelonPatchesFeature {
    fn phase(&self) -> BiomeFeaturePhase {
        BiomeFeaturePhase::Decoration
    }
    fn vertical_range(&self) -> FeatureVerticalRange {
        FeatureVerticalRange::RelativeToSurface { min: 1, max: 1 }
    }
    fn generate(&self, c: &mut BiomeFeatureContext<'_>) {
        let o = c.chunk_position().world_origin();
        for z in o.z..o.z + CHUNK_SIZE {
            for x in o.x..o.x + CHUNK_SIZE {
                if !c.is_target_biome(x, z) {
                    continue;
                }
                let y = c.surface_height(x, z) + 1;
                let p = BlockPos::new(x, y, z);
                if c.hash(p, 0x4d45_4c4f_4e50_4154) % 113 == 0
                    && c.block(p).is_some_and(|b| b.block == BlockId::Air)
                {
                    c.set_block(p, BlockId::Melon);
                }
            }
        }
    }
}
