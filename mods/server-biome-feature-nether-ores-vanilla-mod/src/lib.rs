use bevy_mod::BevyMod;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeFeatureContext, BiomeFeatureId, BiomeFeaturePhase, FeatureVerticalRange, ServerBiomeApi,
    ServerBiomeFeature, ServerBiomeRegistry,
};
use tokio::task::JoinHandle;
use voxel_math_api::{CHUNK_SIZE, LocalBlockPos};
pub fn nether_ores_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new("demo:nether-ores")
}
pub struct ServerBiomeFeatureNetherOresVanillaMod;
impl ServerBiomeFeatureNetherOresVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _nether_quartz_ore: &mut block_nether_quartz_ore::BlockNetherQuartzOreMod,
        _nether_gold_ore: &mut block_nether_gold_ore::BlockNetherGoldOreMod,
        _ancient_debris: &mut block_ancient_debris::BlockAncientDebrisMod,
        _gilded_blackstone: &mut block_gilded_blackstone::BlockGildedBlackstoneMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_feature(nether_ores_feature_id(), NetherOresFeature)
            .expect("nether ore feature id must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
struct NetherOresFeature;
impl ServerBiomeFeature for NetherOresFeature {
    fn phase(&self) -> BiomeFeaturePhase {
        BiomeFeaturePhase::Underground
    }
    fn vertical_range(&self) -> FeatureVerticalRange {
        FeatureVerticalRange::RelativeToSurface { min: -48, max: 0 }
    }
    fn generate(&self, c: &mut BiomeFeatureContext<'_>) {
        let chunk = c.chunk_position();
        let underground = c.definition().terrain.underground;
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let p = LocalBlockPos::new(x, y, z).unwrap().to_world(chunk);
                    if !c.is_target_biome(p.x, p.z)
                        || c.block(p).is_none_or(|b| b.block != underground)
                    {
                        continue;
                    }
                    let h = c.hash(p, 0x4e45_5448_4f52_4553);
                    let roll = h % 2048;
                    let b = if roll < 6 {
                        BlockId::AncientDebris
                    } else if roll < 48 {
                        BlockId::NetherGoldOre
                    } else if roll < 136 {
                        BlockId::NetherQuartzOre
                    } else if roll < 145 {
                        BlockId::GildedBlackstone
                    } else {
                        continue;
                    };
                    c.set_block(p, b);
                }
            }
        }
    }
}
