use bevy_mod::BevyMod;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeFeatureContext, BiomeFeatureId, BiomeFeaturePhase, FeatureVerticalRange, ServerBiomeApi,
    ServerBiomeFeature, ServerBiomeRegistry,
};
use tokio::task::JoinHandle;
use voxel_math_api::{BlockPos, CHUNK_SIZE};

pub const SPARSE_OAK_TREES_FEATURE_ID: &str = "demo:oak-trees-sparse";
pub const DENSE_OAK_TREES_FEATURE_ID: &str = "demo:oak-trees-dense";

pub fn sparse_oak_trees_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new(SPARSE_OAK_TREES_FEATURE_ID)
}

pub fn dense_oak_trees_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new(DENSE_OAK_TREES_FEATURE_ID)
}

pub struct ServerBiomeFeatureOakTreesVanillaMod;

impl ServerBiomeFeatureOakTreesVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _logs: &mut block_oak_log::BlockOakLogMod,
        _leaves: &mut block_oak_leaves::BlockOakLeavesMod,
    ) -> Self {
        let registry = bevy.app.world().resource::<ServerBiomeRegistry>();
        registry
            .register_feature(
                sparse_oak_trees_feature_id(),
                OakTreesFeature {
                    chance_denominator: 151,
                },
            )
            .expect("the sparse oak tree feature id must be unique");
        registry
            .register_feature(
                dense_oak_trees_feature_id(),
                OakTreesFeature {
                    chance_denominator: 29,
                },
            )
            .expect("the dense oak tree feature id must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

struct OakTreesFeature {
    chance_denominator: u64,
}

impl ServerBiomeFeature for OakTreesFeature {
    fn phase(&self) -> BiomeFeaturePhase {
        BiomeFeaturePhase::Decoration
    }

    fn vertical_range(&self) -> FeatureVerticalRange {
        FeatureVerticalRange::RelativeToSurface { min: 1, max: 8 }
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
                let anchor = BlockPos::new(x, surface + 1, z);
                let hash = context.hash(anchor, 0x4f41_4b54_5245_45);
                if hash % self.chance_denominator != 0 {
                    continue;
                }
                let height = 4 + (hash.rotate_left(17) % 3) as i32;
                let crown_y = surface + height;
                for dy in -2i32..=2 {
                    let radius: i32 = if dy.abs() == 2 { 1 } else { 2 };
                    for dz in -radius..=radius {
                        for dx in -radius..=radius {
                            if dx.abs() + dz.abs() > radius + 1 {
                                continue;
                            }
                            let position = BlockPos::new(x + dx, crown_y + dy, z + dz);
                            if context
                                .block(position)
                                .is_some_and(|block| block.block == BlockId::Air)
                            {
                                context.set_block(position, BlockId::OakLeaves);
                            }
                        }
                    }
                }
                for y in (surface + 1)..=crown_y {
                    context.set_block(BlockPos::new(x, y, z), BlockId::OakLog);
                }
            }
        }
    }
}
