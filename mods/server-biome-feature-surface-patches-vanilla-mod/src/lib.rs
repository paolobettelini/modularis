use bevy_mod::BevyMod;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeFeatureContext, BiomeFeatureId, BiomeFeaturePhase, FeatureVerticalRange, ServerBiomeApi,
    ServerBiomeFeature, ServerBiomeRegistry,
};
use tokio::task::JoinHandle;
use voxel_math_api::{BlockPos, CHUNK_SIZE};

pub fn dry_ground_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new("demo:dry-ground-patches")
}
pub fn podzol_patches_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new("demo:podzol-patches")
}
pub fn rocky_patches_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new("demo:rocky-surface-patches")
}
pub fn azalea_shrubs_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new("demo:azalea-shrubs")
}
pub fn mud_patches_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new("demo:mud-patches")
}
pub fn pale_moss_patches_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new("demo:pale-moss-patches")
}
pub fn badlands_bands_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new("demo:badlands-surface-bands")
}
pub fn rooted_patches_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new("demo:rooted-dirt-patches")
}
pub fn mossy_rocks_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new("demo:mossy-rock-patches")
}

#[derive(Clone, Copy)]
enum PatchKind {
    Dry,
    Podzol,
    Rocky,
    Azalea,
    Mud,
    PaleMoss,
    Badlands,
    Rooted,
    Mossy,
}
pub struct ServerBiomeFeatureSurfacePatchesVanillaMod;
impl ServerBiomeFeatureSurfacePatchesVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _coarse_dirt: &mut block_coarse_dirt::BlockCoarseDirtMod,
        _podzol: &mut block_podzol::BlockPodzolMod,
        _andesite: &mut block_andesite::BlockAndesiteMod,
        _diorite: &mut block_diorite::BlockDioriteMod,
        _granite: &mut block_granite::BlockGraniteMod,
        _tuff: &mut block_tuff::BlockTuffMod,
        _azalea_leaves: &mut block_azalea_leaves::BlockAzaleaLeavesMod,
        _flowering_azalea_leaves: &mut block_flowering_azalea_leaves::BlockFloweringAzaleaLeavesMod,
        _mud: &mut block_mud::BlockMudMod,
        _packed_mud: &mut block_packed_mud::BlockPackedMudMod,
        _pale_moss_block: &mut block_pale_moss_block::BlockPaleMossBlockMod,
        _red_sandstone: &mut block_red_sandstone::BlockRedSandstoneMod,
        _red_terracotta: &mut block_red_terracotta::BlockRedTerracottaMod,
        _orange_terracotta: &mut block_orange_terracotta::BlockOrangeTerracottaMod,
        _rooted_dirt: &mut block_rooted_dirt::BlockRootedDirtMod,
        _mossy_cobblestone: &mut block_mossy_cobblestone::BlockMossyCobblestoneMod,
    ) -> Self {
        let r = bevy.app.world().resource::<ServerBiomeRegistry>();
        for (id, kind) in [
            (dry_ground_feature_id(), PatchKind::Dry),
            (podzol_patches_feature_id(), PatchKind::Podzol),
            (rocky_patches_feature_id(), PatchKind::Rocky),
            (azalea_shrubs_feature_id(), PatchKind::Azalea),
            (mud_patches_feature_id(), PatchKind::Mud),
            (pale_moss_patches_feature_id(), PatchKind::PaleMoss),
            (badlands_bands_feature_id(), PatchKind::Badlands),
            (rooted_patches_feature_id(), PatchKind::Rooted),
            (mossy_rocks_feature_id(), PatchKind::Mossy),
        ] {
            r.register_feature(id, SurfacePatchFeature { kind })
                .expect("surface patch feature ids must be unique");
        }
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
struct SurfacePatchFeature {
    kind: PatchKind,
}
impl ServerBiomeFeature for SurfacePatchFeature {
    fn phase(&self) -> BiomeFeaturePhase {
        if matches!(self.kind, PatchKind::Azalea) {
            BiomeFeaturePhase::Decoration
        } else {
            BiomeFeaturePhase::Surface
        }
    }
    fn vertical_range(&self) -> FeatureVerticalRange {
        FeatureVerticalRange::RelativeToSurface { min: 0, max: 1 }
    }
    fn generate(&self, context: &mut BiomeFeatureContext<'_>) {
        let o = context.chunk_position().world_origin();
        for z in o.z..o.z + CHUNK_SIZE {
            for x in o.x..o.x + CHUNK_SIZE {
                if !context.is_target_biome(x, z) {
                    continue;
                }
                let y = context.surface_height(x, z);
                let p = BlockPos::new(x, y, z);
                let h = context.hash(p, 0x5355_5246_5041_5443);
                match self.kind {
                    PatchKind::Dry if h % 13 < 3 => {
                        context.set_block(p, BlockId::CoarseDirt);
                    }
                    PatchKind::Podzol if h % 11 < 4 => {
                        context.set_block(p, BlockId::Podzol);
                    }
                    PatchKind::Rocky if h % 9 < 3 => {
                        let b = match h.rotate_left(7) % 5 {
                            0 => BlockId::Diorite,
                            1 => BlockId::Granite,
                            2 => BlockId::Tuff,
                            _ => BlockId::Andesite,
                        };
                        context.set_block(p, b);
                    }
                    PatchKind::Azalea if h % 37 == 0 => {
                        let above = BlockPos::new(x, y + 1, z);
                        if context
                            .block(above)
                            .is_some_and(|b| b.block == BlockId::Air)
                        {
                            context.set_block(
                                above,
                                if h.rotate_left(17) % 4 == 0 {
                                    BlockId::FloweringAzaleaLeaves
                                } else {
                                    BlockId::AzaleaLeaves
                                },
                            );
                        }
                    }
                    PatchKind::Mud if h % 7 < 3 => {
                        context.set_block(
                            p,
                            if h.rotate_left(5) % 4 == 0 {
                                BlockId::PackedMud
                            } else {
                                BlockId::Mud
                            },
                        );
                    }
                    PatchKind::PaleMoss if h % 5 < 3 => {
                        context.set_block(p, BlockId::PaleMossBlock);
                    }
                    PatchKind::Badlands if h % 7 < 3 => {
                        let b = match h.rotate_left(11) % 4 {
                            0 => BlockId::RedSandstone,
                            1 => BlockId::RedTerracotta,
                            2 => BlockId::OrangeTerracotta,
                            _ => context.definition().terrain.surface,
                        };
                        context.set_block(p, b);
                    }
                    PatchKind::Rooted if h % 11 < 4 => {
                        context.set_block(p, BlockId::RootedDirt);
                    }
                    PatchKind::Mossy if h % 41 == 0 => {
                        context.set_block(p, BlockId::MossyCobblestone);
                    }
                    _ => {}
                }
            }
        }
    }
}
