use bevy_mod::BevyMod;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeFeatureContext, BiomeFeatureId, BiomeFeaturePhase, FeatureVerticalRange, ServerBiomeApi,
    ServerBiomeFeature, ServerBiomeRegistry,
};
use tokio::task::JoinHandle;
use voxel_math_api::{BlockPos, CHUNK_SIZE};
pub const PALE_OAK_TREES_FEATURE_ID: &str = "demo:pale-oak-trees";
pub fn pale_oak_trees_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new(PALE_OAK_TREES_FEATURE_ID)
}
pub struct ServerBiomeFeaturePaleOakTreesVanillaMod;
impl ServerBiomeFeaturePaleOakTreesVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _pale_oak_log: &mut block_pale_oak_log::BlockPaleOakLogMod,
        _pale_oak_leaves: &mut block_pale_oak_leaves::BlockPaleOakLeavesMod,
        _resin_block: &mut block_resin_block::BlockResinBlockMod,
    ) -> Self {
        let registry = bevy.app.world().resource::<ServerBiomeRegistry>();
        registry
            .register_feature(
                pale_oak_trees_feature_id(),
                TreeFeature {
                    chance_denominator: 33,
                },
            )
            .expect("the pale-oak-trees feature id must be unique");
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
                let hash = context.hash(anchor, 0xe06fbea9a596a389);
                if hash % self.chance_denominator != 0 {
                    continue;
                }
                let height = 6 + (hash.rotate_left(7) % 3) as i32;
                let top = surface + height;
                for y in surface + 1..=top {
                    context.set_block(BlockPos::new(x, y, z), BlockId::PaleOakLog);
                    if y > surface + 2 && hash.rotate_left(y as u32 % 31) % 19 == 0 {
                        context.set_block(BlockPos::new(x + 1, y, z), BlockId::ResinBlock);
                    }
                }
                for dy in -2i32..=2 {
                    let radius = if dy.abs() == 2 { 2 } else { 3 };
                    for dz in -radius..=radius {
                        for dx in -radius..=radius {
                            if dx * dx + dz * dz > radius * radius + 3 {
                                continue;
                            }
                            let p = BlockPos::new(x + dx, top + dy, z + dz);
                            if context.block(p).is_some_and(|b| b.block == BlockId::Air) {
                                context.set_block(p, BlockId::PaleOakLeaves);
                            }
                        }
                    }
                }
            }
        }
    }
}
