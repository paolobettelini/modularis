use bevy_mod::BevyMod;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeFeatureContext, BiomeFeatureId, BiomeFeaturePhase, FeatureVerticalRange, ServerBiomeApi,
    ServerBiomeFeature, ServerBiomeRegistry,
};
use tokio::task::JoinHandle;
use voxel_math_api::{BlockPos, CHUNK_SIZE};
pub const DARK_OAK_TREES_FEATURE_ID: &str = "demo:dark-oak-trees";
pub fn dark_oak_trees_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new(DARK_OAK_TREES_FEATURE_ID)
}
pub struct ServerBiomeFeatureDarkOakTreesVanillaMod;
impl ServerBiomeFeatureDarkOakTreesVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _dark_oak_log: &mut block_dark_oak_log::BlockDarkOakLogMod,
        _dark_oak_leaves: &mut block_dark_oak_leaves::BlockDarkOakLeavesMod,
    ) -> Self {
        let registry = bevy.app.world().resource::<ServerBiomeRegistry>();
        registry
            .register_feature(
                dark_oak_trees_feature_id(),
                TreeFeature {
                    chance_denominator: 31,
                },
            )
            .expect("the dark-oak-trees feature id must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
struct TreeFeature {
    chance_denominator: u64,
}
impl ServerBiomeFeature for TreeFeature {
    fn phase(&self) -> BiomeFeaturePhase {
        BiomeFeaturePhase::Decoration
    }
    fn vertical_range(&self) -> FeatureVerticalRange {
        FeatureVerticalRange::RelativeToSurface { min: -2, max: 16 }
    }
    fn generate(&self, context: &mut BiomeFeatureContext<'_>) {
        let origin = context.chunk_position().world_origin();
        for z in (origin.z - 4)..=(origin.z + CHUNK_SIZE + 3) {
            for x in (origin.x - 4)..=(origin.x + CHUNK_SIZE + 3) {
                if !context.is_target_biome(x, z)
                    || (x as i64 * x as i64 + z as i64 * z as i64) < 144
                {
                    continue;
                }
                let surface = context.surface_height(x, z);
                let anchor = BlockPos::new(x, surface + 1, z);
                let hash = context.hash(anchor, 0xb3a278d546f3ca19);
                if hash % self.chance_denominator != 0 {
                    continue;
                }
                let height = 6 + (hash.rotate_left(7) % 3) as i32;
                let top = surface + height;
                for y in surface + 1..=top {
                    for dz in 0..=1 {
                        for dx in 0..=1 {
                            context
                                .set_block(BlockPos::new(x + dx, y, z + dz), BlockId::DarkOakLog);
                        }
                    }
                }
                for dy in -1i32..=2 {
                    let radius: i32 = if dy == 2 { 2 } else { 3 };
                    for dz in -radius..=radius {
                        for dx in -radius..=radius {
                            if dx.abs() + dz.abs() > radius + 2 {
                                continue;
                            }
                            let p = BlockPos::new(x + dx, top + dy, z + dz);
                            if context.block(p).is_some_and(|b| b.block == BlockId::Air) {
                                context.set_block(p, BlockId::DarkOakLeaves);
                            }
                        }
                    }
                }
            }
        }
    }
}
