use bevy_mod::BevyMod;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeFeatureContext, BiomeFeatureId, BiomeFeaturePhase, FeatureVerticalRange, ServerBiomeApi,
    ServerBiomeFeature, ServerBiomeRegistry,
};
use tokio::task::JoinHandle;
use voxel_math_api::{BlockPos, CHUNK_SIZE};
pub const SPARSE_SPRUCE_TREES_FEATURE_ID: &str = "demo:spruce-trees-sparse";
pub const DENSE_SPRUCE_TREES_FEATURE_ID: &str = "demo:spruce-trees-dense";
pub fn sparse_spruce_trees_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new(SPARSE_SPRUCE_TREES_FEATURE_ID)
}
pub fn dense_spruce_trees_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new(DENSE_SPRUCE_TREES_FEATURE_ID)
}
pub struct ServerBiomeFeatureSpruceTreesVanillaMod;
impl ServerBiomeFeatureSpruceTreesVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _spruce_log: &mut block_spruce_log::BlockSpruceLogMod,
        _spruce_leaves: &mut block_spruce_leaves::BlockSpruceLeavesMod,
    ) -> Self {
        let registry = bevy.app.world().resource::<ServerBiomeRegistry>();
        registry
            .register_feature(
                sparse_spruce_trees_feature_id(),
                TreeFeature {
                    chance_denominator: 83,
                },
            )
            .expect("the spruce-trees-sparse feature id must be unique");
        registry
            .register_feature(
                dense_spruce_trees_feature_id(),
                TreeFeature {
                    chance_denominator: 37,
                },
            )
            .expect("the spruce-trees-dense feature id must be unique");
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
                let hash = context.hash(anchor, 0xf816a04c772b3d93);
                if hash % self.chance_denominator != 0 {
                    continue;
                }
                let height = 6 + (hash.rotate_left(9) % 4) as i32;
                let top = surface + height;
                for y in surface + 1..=top {
                    context.set_block(BlockPos::new(x, y, z), BlockId::SpruceLog);
                }
                for layer in 0..=5i32 {
                    let y = top - layer;
                    let radius = if layer == 0 {
                        0
                    } else {
                        1 + (layer / 2).min(2)
                    };
                    for dz in -radius..=radius {
                        for dx in -radius..=radius {
                            if dx.abs() + dz.abs() > radius + 1 {
                                continue;
                            }
                            let p = BlockPos::new(x + dx, y, z + dz);
                            if context.block(p).is_some_and(|b| b.block == BlockId::Air) {
                                context.set_block(p, BlockId::SpruceLeaves);
                            }
                        }
                    }
                }
            }
        }
    }
}
