use bevy_mod::BevyMod;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeFeatureContext, BiomeFeatureId, BiomeFeaturePhase, FeatureVerticalRange, ServerBiomeApi,
    ServerBiomeFeature, ServerBiomeRegistry,
};
use tokio::task::JoinHandle;
use voxel_math_api::{BlockPos, CHUNK_SIZE};

pub const BOULDERS_FEATURE_ID: &str = "demo:rock-boulders";
pub fn boulders_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new(BOULDERS_FEATURE_ID)
}
pub struct ServerBiomeFeatureBouldersVanillaMod;
impl ServerBiomeFeatureBouldersVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _gravel: &mut block_gravel::BlockGravelMod,
        _andesite: &mut block_andesite::BlockAndesiteMod,
        _diorite: &mut block_diorite::BlockDioriteMod,
        _granite: &mut block_granite::BlockGraniteMod,
        _tuff: &mut block_tuff::BlockTuffMod,
        _cobbled_deepslate: &mut block_cobbled_deepslate::BlockCobbledDeepslateMod,
        _mossy_cobblestone: &mut block_mossy_cobblestone::BlockMossyCobblestoneMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_feature(boulders_feature_id(), BouldersFeature)
            .expect("the vanilla boulder feature id must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
struct BouldersFeature;
impl ServerBiomeFeature for BouldersFeature {
    fn phase(&self) -> BiomeFeaturePhase {
        BiomeFeaturePhase::Decoration
    }
    fn vertical_range(&self) -> FeatureVerticalRange {
        FeatureVerticalRange::RelativeToSurface { min: 0, max: 4 }
    }
    fn generate(&self, context: &mut BiomeFeatureContext<'_>) {
        let origin = context.chunk_position().world_origin();
        for z in (origin.z - 2)..=(origin.z + CHUNK_SIZE + 1) {
            for x in (origin.x - 2)..=(origin.x + CHUNK_SIZE + 1) {
                if !context.is_target_biome(x, z)
                    || (x as i64 * x as i64 + z as i64 * z as i64) < 144
                {
                    continue;
                }
                let surface = context.surface_height(x, z);
                let anchor = BlockPos::new(x, surface, z);
                let hash = context.hash(anchor, 0x424f_554c_4445_52);
                if hash % 71 != 0 {
                    continue;
                }
                let radius = 1 + (hash.rotate_left(11) % 3) as i32;
                for dy in 0..=radius {
                    for dz in -radius..=radius {
                        for dx in -radius..=radius {
                            if dx * dx + dz * dz + dy * dy > radius * radius + 1 {
                                continue;
                            }
                            let h = context.hash(
                                BlockPos::new(x + dx, surface + dy, z + dz),
                                0x524f_434b_5041_4c45,
                            );
                            let block = match h % 10 {
                                0 => BlockId::Gravel,
                                1 => BlockId::Diorite,
                                2 => BlockId::Granite,
                                3 => BlockId::Tuff,
                                4 => BlockId::CobbledDeepslate,
                                5 => BlockId::MossyCobblestone,
                                _ => BlockId::Andesite,
                            };
                            context.set_block(BlockPos::new(x + dx, surface + dy, z + dz), block);
                        }
                    }
                }
            }
        }
    }
}
