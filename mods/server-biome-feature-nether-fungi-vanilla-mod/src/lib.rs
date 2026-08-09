use bevy_mod::BevyMod;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeFeatureContext, BiomeFeatureId, BiomeFeaturePhase, FeatureVerticalRange, ServerBiomeApi,
    ServerBiomeFeature, ServerBiomeRegistry,
};
use tokio::task::JoinHandle;
use voxel_math_api::{BlockPos, CHUNK_SIZE};
pub fn crimson_fungi_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new("demo:crimson-fungi")
}
pub fn warped_fungi_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new("demo:warped-fungi")
}
#[derive(Clone, Copy)]
enum Kind {
    Crimson,
    Warped,
}
pub struct ServerBiomeFeatureNetherFungiVanillaMod;
impl ServerBiomeFeatureNetherFungiVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _crimson_stem: &mut block_crimson_stem::BlockCrimsonStemMod,
        _nether_wart_block: &mut block_nether_wart_block::BlockNetherWartBlockMod,
        _warped_stem: &mut block_warped_stem::BlockWarpedStemMod,
        _warped_wart_block: &mut block_warped_wart_block::BlockWarpedWartBlockMod,
        _shroomlight: &mut block_shroomlight::BlockShroomlightMod,
    ) -> Self {
        let r = bevy.app.world().resource::<ServerBiomeRegistry>();
        r.register_feature(
            crimson_fungi_feature_id(),
            NetherFungiFeature {
                kind: Kind::Crimson,
            },
        )
        .expect("crimson fungi feature id must be unique");
        r.register_feature(
            warped_fungi_feature_id(),
            NetherFungiFeature { kind: Kind::Warped },
        )
        .expect("warped fungi feature id must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
struct NetherFungiFeature {
    kind: Kind,
}
impl ServerBiomeFeature for NetherFungiFeature {
    fn phase(&self) -> BiomeFeaturePhase {
        BiomeFeaturePhase::Decoration
    }
    fn vertical_range(&self) -> FeatureVerticalRange {
        FeatureVerticalRange::RelativeToSurface { min: 1, max: 11 }
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
                let h = c.hash(a, 0x4e45_5448_4655_4e47);
                if h % 31 != 0 {
                    continue;
                }
                let (stem, cap) = match self.kind {
                    Kind::Crimson => (BlockId::CrimsonStem, BlockId::NetherWartBlock),
                    Kind::Warped => (BlockId::WarpedStem, BlockId::WarpedWartBlock),
                };
                let height = 5 + (h.rotate_left(8) % 4) as i32;
                for y in s + 1..=s + height {
                    c.set_block(BlockPos::new(x, y, z), stem);
                }
                for dz in -2i32..=2 {
                    for dx in -2i32..=2 {
                        if dx.abs() + dz.abs() > 3 {
                            continue;
                        }
                        let p = BlockPos::new(x + dx, s + height, z + dz);
                        c.set_block(
                            p,
                            if h.rotate_left((dx.abs() + dz.abs()) as u32) % 17 == 0 {
                                BlockId::Shroomlight
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
