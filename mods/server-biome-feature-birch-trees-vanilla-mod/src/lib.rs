use bevy_mod::BevyMod;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeFeatureContext, BiomeFeatureId, BiomeFeaturePhase, FeatureVerticalRange, ServerBiomeApi,
    ServerBiomeFeature, ServerBiomeRegistry,
};
use tokio::task::JoinHandle;
use voxel_math_api::{BlockPos, CHUNK_SIZE};

pub const BIRCH_TREES_FEATURE_ID: &str = "demo:birch-trees";

pub fn birch_trees_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new(BIRCH_TREES_FEATURE_ID)
}

pub struct ServerBiomeFeatureBirchTreesVanillaMod;

impl ServerBiomeFeatureBirchTreesVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _logs: &mut block_birch_log::BlockBirchLogMod,
        _leaves: &mut block_birch_leaves::BlockBirchLeavesMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_feature(birch_trees_feature_id(), BirchTreesFeature)
            .expect("the birch tree feature id must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

struct BirchTreesFeature;

impl ServerBiomeFeature for BirchTreesFeature {
    fn phase(&self) -> BiomeFeaturePhase {
        BiomeFeaturePhase::Decoration
    }

    fn vertical_range(&self) -> FeatureVerticalRange {
        FeatureVerticalRange::RelativeToSurface { min: 1, max: 9 }
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
                let hash = context.hash(anchor, 0x4249_5243_4854_5245);
                if hash % 37 != 0 {
                    continue;
                }
                let height = 5 + (hash.rotate_left(17) % 3) as i32;
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
                                context.set_block(position, BlockId::BirchLeaves);
                            }
                        }
                    }
                }
                for y in (surface + 1)..=crown_y {
                    context.set_block(BlockPos::new(x, y, z), BlockId::BirchLog);
                }
            }
        }
    }
}
