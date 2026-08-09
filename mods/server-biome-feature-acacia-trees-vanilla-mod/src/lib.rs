use bevy_mod::BevyMod;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeFeatureContext, BiomeFeatureId, BiomeFeaturePhase, FeatureVerticalRange, ServerBiomeApi,
    ServerBiomeFeature, ServerBiomeRegistry,
};
use tokio::task::JoinHandle;
use voxel_math_api::{BlockPos, CHUNK_SIZE};
pub const SPARSE_ACACIA_TREES_FEATURE_ID: &str = "demo:acacia-trees-sparse";
pub const DENSE_ACACIA_TREES_FEATURE_ID: &str = "demo:acacia-trees-dense";
pub fn sparse_acacia_trees_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new(SPARSE_ACACIA_TREES_FEATURE_ID)
}
pub fn dense_acacia_trees_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new(DENSE_ACACIA_TREES_FEATURE_ID)
}
pub struct ServerBiomeFeatureAcaciaTreesVanillaMod;
impl ServerBiomeFeatureAcaciaTreesVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _acacia_log: &mut block_acacia_log::BlockAcaciaLogMod,
        _acacia_leaves: &mut block_acacia_leaves::BlockAcaciaLeavesMod,
    ) -> Self {
        let registry = bevy.app.world().resource::<ServerBiomeRegistry>();
        registry
            .register_feature(
                sparse_acacia_trees_feature_id(),
                TreeFeature {
                    chance_denominator: 101,
                },
            )
            .expect("the acacia-trees-sparse feature id must be unique");
        registry
            .register_feature(
                dense_acacia_trees_feature_id(),
                TreeFeature {
                    chance_denominator: 43,
                },
            )
            .expect("the acacia-trees-dense feature id must be unique");
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
                let hash = context.hash(anchor, 0x609e7bbff919793b);
                if hash % self.chance_denominator != 0 {
                    continue;
                }
                let height = 4 + (hash.rotate_left(9) % 3) as i32;
                let top = surface + height;
                for y in surface + 1..=top {
                    context.set_block(BlockPos::new(x, y, z), BlockId::AcaciaLog);
                }
                for dz in -2i32..=2 {
                    for dx in -2i32..=2 {
                        if dx.abs() + dz.abs() > 3 {
                            continue;
                        }
                        let p = BlockPos::new(x + dx, top, z + dz);
                        if context.block(p).is_some_and(|b| b.block == BlockId::Air) {
                            context.set_block(p, BlockId::AcaciaLeaves);
                        }
                    }
                }
                if hash % 3 == 0 {
                    for dx in 2..=3 {
                        context.set_block(BlockPos::new(x + dx, top - 1, z), BlockId::AcaciaLog);
                    }
                    for dz in -2i32..=2 {
                        for dx in 1i32..=4 {
                            if (dx - 3).abs() + dz.abs() <= 2 {
                                let p = BlockPos::new(x + dx, top, z + dz);
                                if context.block(p).is_some_and(|b| b.block == BlockId::Air) {
                                    context.set_block(p, BlockId::AcaciaLeaves);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
