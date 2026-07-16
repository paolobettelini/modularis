use bevy_mod::BevyMod;
use generated_biome_registry::BiomeId;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeClimate, BiomeDefinition, BiomeTerrain, BiomeVisuals, ServerBiomeApi, ServerBiomeRegistry,
};
use server_biome_feature_cacti_vanilla_mod::{ServerBiomeFeatureCactiVanillaMod, cacti_feature_id};
use server_biome_feature_caves_vanilla_mod::{ServerBiomeFeatureCavesVanillaMod, caves_feature_id};
use server_biome_feature_ores_vanilla_mod::{ServerBiomeFeatureOresVanillaMod, ores_feature_id};
use tokio::task::JoinHandle;

pub struct ServerBiomeDesertVanillaMod;

impl ServerBiomeDesertVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_desert::BiomeDesertMod,
        _caves: &mut ServerBiomeFeatureCavesVanillaMod,
        _ores: &mut ServerBiomeFeatureOresVanillaMod,
        _cacti: &mut ServerBiomeFeatureCactiVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::Desert,
                name: "Dry Desert",
                climate: BiomeClimate {
                    temperature: 0.92,
                    humidity: 0.12,
                    continentalness: 0.42,
                    has_precipitation: false,
                    downfall: 0.02,
                },
                terrain: BiomeTerrain {
                    base_height: 5.2,
                    height_variation: 3.8,
                    detail_variation: 1.4,
                    surface: BlockId::Sand,
                    subsurface: BlockId::Sand,
                    underground: BlockId::Stone,
                    subsurface_depth: 5,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.60, 0.76, 0.96],
                    fog_color: [0.88, 0.78, 0.60],
                    water_color: [0.22, 0.44, 0.70],
                    grass_tint: [0.78, 0.72, 0.38],
                    foliage_tint: [0.68, 0.66, 0.34],
                },
                features: vec![caves_feature_id(), ores_feature_id(), cacti_feature_id()],
            })
            .expect("the desert biome definition must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
