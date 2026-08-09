use bevy_mod::BevyMod;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeFeatureContext, BiomeFeatureId, BiomeFeaturePhase, FeatureVerticalRange, ServerBiomeApi,
    ServerBiomeFeature, ServerBiomeRegistry,
};
use tokio::task::JoinHandle;
use voxel_math_api::{BlockPos, CHUNK_SIZE};
pub const MANGROVE_TREES_FEATURE_ID: &str = "demo:mangrove-trees";
pub fn mangrove_trees_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new(MANGROVE_TREES_FEATURE_ID)
}
pub struct ServerBiomeFeatureMangroveTreesVanillaMod;
impl ServerBiomeFeatureMangroveTreesVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _mangrove_log: &mut block_mangrove_log::BlockMangroveLogMod,
        _mangrove_leaves: &mut block_mangrove_leaves::BlockMangroveLeavesMod,
        _mangrove_roots: &mut block_mangrove_roots::BlockMangroveRootsMod,
        _muddy_mangrove_roots: &mut block_muddy_mangrove_roots::BlockMuddyMangroveRootsMod,
    ) -> Self {
        let registry = bevy.app.world().resource::<ServerBiomeRegistry>();
        registry
            .register_feature(
                mangrove_trees_feature_id(),
                TreeFeature {
                    chance_denominator: 27,
                },
            )
            .expect("the mangrove-trees feature id must be unique");
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
                let hash = context.hash(anchor, 0x189002513b3be856);
                if hash % self.chance_denominator != 0 {
                    continue;
                }
                let height = 5 + (hash.rotate_left(8) % 4) as i32;
                let top = surface + height;
                for dz in -1i32..=1 {
                    for dx in -1i32..=1 {
                        if dx == 0 || dz == 0 {
                            context.set_block(
                                BlockPos::new(x + dx, surface, z + dz),
                                BlockId::MangroveRoots,
                            );
                            context.set_block(
                                BlockPos::new(x + dx, surface - 1, z + dz),
                                BlockId::MuddyMangroveRoots,
                            );
                        }
                    }
                }
                for y in surface + 1..=top {
                    context.set_block(BlockPos::new(x, y, z), BlockId::MangroveLog);
                }
                for dy in -1i32..=2 {
                    let radius: i32 = if dy == 2 { 1 } else { 3 };
                    for dz in -radius..=radius {
                        for dx in -radius..=radius {
                            if dx.abs() + dz.abs() > radius + 2 {
                                continue;
                            }
                            let p = BlockPos::new(x + dx, top + dy, z + dz);
                            if context.block(p).is_some_and(|b| b.block == BlockId::Air) {
                                context.set_block(p, BlockId::MangroveLeaves);
                            }
                        }
                    }
                }
            }
        }
    }
}
