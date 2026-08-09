use bevy_mod::BevyMod;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeFeatureContext, BiomeFeatureId, BiomeFeaturePhase, FeatureVerticalRange, ServerBiomeApi,
    ServerBiomeFeature, ServerBiomeRegistry,
};
use tokio::task::JoinHandle;
use voxel_math_api::{BlockPos, CHUNK_SIZE};
pub const JUNGLE_TREES_FEATURE_ID: &str = "demo:jungle-trees";
pub fn jungle_trees_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new(JUNGLE_TREES_FEATURE_ID)
}
pub struct ServerBiomeFeatureJungleTreesVanillaMod;
impl ServerBiomeFeatureJungleTreesVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _jungle_log: &mut block_jungle_log::BlockJungleLogMod,
        _jungle_leaves: &mut block_jungle_leaves::BlockJungleLeavesMod,
    ) -> Self {
        let registry = bevy.app.world().resource::<ServerBiomeRegistry>();
        registry
            .register_feature(
                jungle_trees_feature_id(),
                TreeFeature {
                    chance_denominator: 29,
                },
            )
            .expect("the jungle-trees feature id must be unique");
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
                let hash = context.hash(anchor, 0x778267752d7f95dc);
                if hash % self.chance_denominator != 0 {
                    continue;
                }
                let height = 8 + (hash.rotate_left(11) % 5) as i32;
                let top = surface + height;
                for y in surface + 1..=top {
                    context.set_block(BlockPos::new(x, y, z), BlockId::JungleLog);
                }
                for dy in -2i32..=2 {
                    let radius = if dy.abs() == 2 { 2 } else { 3 };
                    for dz in -radius..=radius {
                        for dx in -radius..=radius {
                            if dx * dx + dz * dz > radius * radius + 2 {
                                continue;
                            }
                            let p = BlockPos::new(x + dx, top + dy, z + dz);
                            if context.block(p).is_some_and(|b| b.block == BlockId::Air) {
                                context.set_block(p, BlockId::JungleLeaves);
                            }
                        }
                    }
                }
            }
        }
    }
}
