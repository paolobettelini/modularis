use bevy_mod::BevyMod;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeFeatureContext, BiomeFeatureId, BiomeFeaturePhase, FeatureVerticalRange, ServerBiomeApi,
    ServerBiomeFeature, ServerBiomeRegistry,
};
use tokio::task::JoinHandle;
use voxel_math_api::{BlockPos, CHUNK_SIZE};
pub const ICE_PATCHES_FEATURE_ID: &str = "demo:ice-patches";
pub fn ice_patches_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new(ICE_PATCHES_FEATURE_ID)
}
pub struct ServerBiomeFeatureIcePatchesVanillaMod;
impl ServerBiomeFeatureIcePatchesVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _packed_ice: &mut block_packed_ice::BlockPackedIceMod,
        _ice: &mut block_ice::BlockIceMod,
        _blue_ice: &mut block_blue_ice::BlockBlueIceMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_feature(ice_patches_feature_id(), IcePatchesFeature)
            .expect("the vanilla ice patch feature id must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
struct IcePatchesFeature;
impl ServerBiomeFeature for IcePatchesFeature {
    fn phase(&self) -> BiomeFeaturePhase {
        BiomeFeaturePhase::Surface
    }
    fn vertical_range(&self) -> FeatureVerticalRange {
        FeatureVerticalRange::RelativeToSurface { min: 0, max: 1 }
    }
    fn generate(&self, context: &mut BiomeFeatureContext<'_>) {
        let origin = context.chunk_position().world_origin();
        for z in origin.z..origin.z + CHUNK_SIZE {
            for x in origin.x..origin.x + CHUNK_SIZE {
                if !context.is_target_biome(x, z) {
                    continue;
                }
                let y = context.surface_height(x, z);
                let p = BlockPos::new(x, y, z);
                let h = context.hash(p, 0x4943_4550_4154_4348);
                if h % 17 < 5 {
                    let block = match h.rotate_left(13) % 12 {
                        0 => BlockId::BlueIce,
                        1 | 2 | 3 => BlockId::Ice,
                        _ => BlockId::PackedIce,
                    };
                    context.set_block(p, block);
                }
            }
        }
    }
}
