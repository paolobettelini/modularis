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
        _ice: &mut block_packed_ice::BlockPackedIceMod,
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
        FeatureVerticalRange::RelativeToSurface { min: 0, max: 0 }
    }

    fn generate(&self, context: &mut BiomeFeatureContext<'_>) {
        let origin = context.chunk_position().world_origin();
        for z in origin.z..(origin.z + CHUNK_SIZE) {
            for x in origin.x..(origin.x + CHUNK_SIZE) {
                if !context.is_target_biome(x, z) {
                    continue;
                }
                let surface = context.surface_height(x, z);
                let position = BlockPos::new(x, surface, z);
                if context.hash(position, 0x4943_4550_4154_4348) % 13 < 3 {
                    context.set_block(position, BlockId::PackedIce);
                }
            }
        }
    }
}
