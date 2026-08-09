use bevy_mod::BevyMod;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeFeatureContext, BiomeFeatureId, BiomeFeaturePhase, FeatureVerticalRange, ServerBiomeApi,
    ServerBiomeFeature, ServerBiomeRegistry,
};
use tokio::task::JoinHandle;
use voxel_math_api::{BlockPos, CHUNK_SIZE};
pub fn mushroom_groves_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new("demo:mushroom-groves")
}
pub struct ServerBiomeFeatureMushroomGrovesVanillaMod;
impl ServerBiomeFeatureMushroomGrovesVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _red_mushroom_block: &mut block_red_mushroom_block::BlockRedMushroomBlockMod,
        _brown_mushroom_block: &mut block_brown_mushroom_block::BlockBrownMushroomBlockMod,
        _mushroom_block_inside: &mut block_mushroom_block_inside::BlockMushroomBlockInsideMod,
        _mushroom_stem: &mut block_mushroom_stem::BlockMushroomStemMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_feature(mushroom_groves_feature_id(), MushroomGrovesFeature)
            .expect("mushroom grove feature id must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
struct MushroomGrovesFeature;
impl ServerBiomeFeature for MushroomGrovesFeature {
    fn phase(&self) -> BiomeFeaturePhase {
        BiomeFeaturePhase::Decoration
    }
    fn vertical_range(&self) -> FeatureVerticalRange {
        FeatureVerticalRange::RelativeToSurface { min: 1, max: 9 }
    }
    fn generate(&self, c: &mut BiomeFeatureContext<'_>) {
        let o = c.chunk_position().world_origin();
        for z in (o.z - 3)..=o.z + CHUNK_SIZE + 2 {
            for x in (o.x - 3)..=o.x + CHUNK_SIZE + 2 {
                if !c.is_target_biome(x, z) {
                    continue;
                }
                let s = c.surface_height(x, z);
                let a = BlockPos::new(x, s + 1, z);
                let h = c.hash(a, 0x4d55_5348_524f_4f4d);
                if h % 43 != 0 {
                    continue;
                }
                let height = 4 + (h.rotate_left(8) % 3) as i32;
                for y in s + 1..=s + height {
                    c.set_block(BlockPos::new(x, y, z), BlockId::MushroomStem);
                }
                let cap = if h % 2 == 0 {
                    BlockId::RedMushroomBlock
                } else {
                    BlockId::BrownMushroomBlock
                };
                for dz in -2i32..=2 {
                    for dx in -2i32..=2 {
                        if dx.abs() + dz.abs() > 3 {
                            continue;
                        }
                        let p = BlockPos::new(x + dx, s + height, z + dz);
                        c.set_block(
                            p,
                            if dx == 0 && dz == 0 {
                                BlockId::MushroomBlockInside
                            } else {
                                cap
                            },
                        );
                    }
                }
            }
        }
    }
}
