use bevy_mod::BevyMod;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeFeatureContext, BiomeFeatureId, BiomeFeaturePhase, FeatureVerticalRange, ServerBiomeApi,
    ServerBiomeFeature, ServerBiomeRegistry,
};
use tokio::task::JoinHandle;
use voxel_math_api::{BlockPos, CHUNK_SIZE};
pub fn golden_lights_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new("demo:aether-golden-lights")
}
pub fn pearlescent_lights_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new("demo:aether-pearlescent-lights")
}
pub fn verdant_lights_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new("demo:aether-verdant-lights")
}
pub fn tempest_lights_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new("demo:aether-tempest-lights")
}
#[derive(Clone, Copy)]
enum Kind {
    Golden,
    Pearlescent,
    Verdant,
    Tempest,
}
pub struct ServerBiomeFeatureAetherLightsVanillaMod;
impl ServerBiomeFeatureAetherLightsVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _ochre_froglight: &mut block_ochre_froglight::BlockOchreFroglightMod,
        _pearlescent_froglight: &mut block_pearlescent_froglight::BlockPearlescentFroglightMod,
        _verdant_froglight: &mut block_verdant_froglight::BlockVerdantFroglightMod,
        _sea_lantern: &mut block_sea_lantern::BlockSeaLanternMod,
        _honeycomb_block: &mut block_honeycomb_block::BlockHoneycombBlockMod,
    ) -> Self {
        let r = bevy.app.world().resource::<ServerBiomeRegistry>();
        for (id, kind) in [
            (golden_lights_feature_id(), Kind::Golden),
            (pearlescent_lights_feature_id(), Kind::Pearlescent),
            (verdant_lights_feature_id(), Kind::Verdant),
            (tempest_lights_feature_id(), Kind::Tempest),
        ] {
            r.register_feature(id, AetherLightsFeature { kind })
                .expect("aether light feature ids must be unique");
        }
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
struct AetherLightsFeature {
    kind: Kind,
}
impl ServerBiomeFeature for AetherLightsFeature {
    fn phase(&self) -> BiomeFeaturePhase {
        BiomeFeaturePhase::Decoration
    }
    fn vertical_range(&self) -> FeatureVerticalRange {
        FeatureVerticalRange::RelativeToSurface { min: 0, max: 6 }
    }
    fn generate(&self, c: &mut BiomeFeatureContext<'_>) {
        let o = c.chunk_position().world_origin();
        for z in (o.z - 1)..=o.z + CHUNK_SIZE {
            for x in (o.x - 1)..=o.x + CHUNK_SIZE {
                if !c.is_target_biome(x, z) {
                    continue;
                }
                let s = c.surface_height(x, z);
                let h = c.hash(BlockPos::new(x, s, z), 0x4145_5448_4c49_5445);
                if h % 53 != 0 {
                    continue;
                }
                match self.kind {
                    Kind::Golden => {
                        c.set_block(BlockPos::new(x, s, z), BlockId::HoneycombBlock);
                        c.set_block(BlockPos::new(x, s + 1, z), BlockId::OchreFroglight);
                    }
                    Kind::Pearlescent => {
                        for y in 1..=2 + (h % 3) as i32 {
                            c.set_block(BlockPos::new(x, s + y, z), BlockId::PearlescentFroglight);
                        }
                    }
                    Kind::Verdant => {
                        c.set_block(BlockPos::new(x, s + 1, z), BlockId::VerdantFroglight);
                    }
                    Kind::Tempest => {
                        let height = 2 + (h % 4) as i32;
                        for y in 1..=height {
                            c.set_block(
                                BlockPos::new(x, s + y, z),
                                if y == height {
                                    BlockId::SeaLantern
                                } else {
                                    BlockId::PearlescentFroglight
                                },
                            );
                        }
                    }
                }
            }
        }
    }
}
