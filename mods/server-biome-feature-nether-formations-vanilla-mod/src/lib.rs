use bevy_mod::BevyMod;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeFeatureContext, BiomeFeatureId, BiomeFeaturePhase, FeatureVerticalRange, ServerBiomeApi,
    ServerBiomeFeature, ServerBiomeRegistry,
};
use tokio::task::JoinHandle;
use voxel_math_api::{BlockPos, CHUNK_SIZE};
pub fn bone_fields_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new("demo:bone-fields")
}
pub fn sulfur_spires_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new("demo:sulfur-spires")
}
pub fn cinnabar_columns_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new("demo:cinnabar-columns")
}
pub fn blackstone_outcrops_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new("demo:blackstone-outcrops")
}
#[derive(Clone, Copy)]
enum Kind {
    Bone,
    Sulfur,
    Cinnabar,
    Blackstone,
}
pub struct ServerBiomeFeatureNetherFormationsVanillaMod;
impl ServerBiomeFeatureNetherFormationsVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _bone_block: &mut block_bone_block::BlockBoneBlockMod,
        _sulfur: &mut block_sulfur::BlockSulfurMod,
        _potent_sulfur: &mut block_potent_sulfur::BlockPotentSulfurMod,
        _polished_sulfur: &mut block_polished_sulfur::BlockPolishedSulfurMod,
        _cinnabar: &mut block_cinnabar::BlockCinnabarMod,
        _polished_cinnabar: &mut block_polished_cinnabar::BlockPolishedCinnabarMod,
        _chiseled_cinnabar: &mut block_chiseled_cinnabar::BlockChiseledCinnabarMod,
        _polished_blackstone: &mut block_polished_blackstone::BlockPolishedBlackstoneMod,
        _gilded_blackstone: &mut block_gilded_blackstone::BlockGildedBlackstoneMod,
    ) -> Self {
        let r = bevy.app.world().resource::<ServerBiomeRegistry>();
        for (id, kind) in [
            (bone_fields_feature_id(), Kind::Bone),
            (sulfur_spires_feature_id(), Kind::Sulfur),
            (cinnabar_columns_feature_id(), Kind::Cinnabar),
            (blackstone_outcrops_feature_id(), Kind::Blackstone),
        ] {
            r.register_feature(id, FormationFeature { kind })
                .expect("nether formation feature ids must be unique");
        }
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
struct FormationFeature {
    kind: Kind,
}
impl ServerBiomeFeature for FormationFeature {
    fn phase(&self) -> BiomeFeaturePhase {
        BiomeFeaturePhase::Decoration
    }
    fn vertical_range(&self) -> FeatureVerticalRange {
        FeatureVerticalRange::RelativeToSurface { min: 0, max: 10 }
    }
    fn generate(&self, c: &mut BiomeFeatureContext<'_>) {
        let o = c.chunk_position().world_origin();
        for z in (o.z - 2)..=o.z + CHUNK_SIZE + 1 {
            for x in (o.x - 2)..=o.x + CHUNK_SIZE + 1 {
                if !c.is_target_biome(x, z) {
                    continue;
                }
                let s = c.surface_height(x, z);
                let h = c.hash(BlockPos::new(x, s, z), 0x4e46_4f52_4d53_3031);
                match self.kind {
                    Kind::Bone if h % 89 == 0 => {
                        let height = 3 + (h.rotate_left(9) % 5) as i32;
                        for y in 1..=height {
                            c.set_block(BlockPos::new(x, s + y, z), BlockId::BoneBlock);
                        }
                    }
                    Kind::Sulfur if h % 43 == 0 => {
                        let height = 2 + (h.rotate_left(8) % 6) as i32;
                        for y in 0..=height {
                            let b = if y == height && h % 5 == 0 {
                                BlockId::PotentSulfur
                            } else if y == 0 {
                                BlockId::PolishedSulfur
                            } else {
                                BlockId::Sulfur
                            };
                            c.set_block(BlockPos::new(x, s + y, z), b);
                        }
                    }
                    Kind::Cinnabar if h % 47 == 0 => {
                        let height = 2 + (h.rotate_left(7) % 5) as i32;
                        for y in 0..=height {
                            let b = if y == height {
                                BlockId::ChiseledCinnabar
                            } else if y == 0 {
                                BlockId::PolishedCinnabar
                            } else {
                                BlockId::Cinnabar
                            };
                            c.set_block(BlockPos::new(x, s + y, z), b);
                        }
                    }
                    Kind::Blackstone if h % 31 == 0 => {
                        for dz in -1i32..=1 {
                            for dx in -1i32..=1 {
                                let b = if h.rotate_left((dx.abs() + dz.abs()) as u32) % 13 == 0 {
                                    BlockId::GildedBlackstone
                                } else {
                                    BlockId::PolishedBlackstone
                                };
                                c.set_block(BlockPos::new(x + dx, s, z + dz), b);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
