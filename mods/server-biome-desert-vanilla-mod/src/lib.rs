use bevy_mod::BevyMod;
use generated_biome_registry::BiomeId;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeClimate, BiomeDefinition, BiomeTerrain, BiomeVisuals, Dimension, ServerBiomeApi,
    ServerBiomeRegistry,
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
        _caves: &mut server_biome_feature_caves_vanilla_mod::ServerBiomeFeatureCavesVanillaMod,
        _ores: &mut server_biome_feature_ores_vanilla_mod::ServerBiomeFeatureOresVanillaMod,
        _cacti: &mut server_biome_feature_cacti_vanilla_mod::ServerBiomeFeatureCactiVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::Desert,
                dimension: Dimension::Overworld,
                name: "Dune Desert",
                climate: BiomeClimate {
                    temperature: 0.93,
                    humidity: 0.05,
                    continentalness: 0.46,
                    has_precipitation: false,
                    downfall: 0.00,
                },
                terrain: BiomeTerrain {
                    base_height: 5.80,
                    height_variation: 4.60,
                    detail_variation: 1.60,
                    surface: BlockId::Sand,
                    subsurface: BlockId::Sandstone,
                    underground: BlockId::Stone,
                    subsurface_depth: 6,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.72, 0.82, 0.98],
                    fog_color: [0.92, 0.76, 0.52],
                    water_color: [0.20, 0.44, 0.68],
                    grass_tint: [0.72, 0.68, 0.30],
                    foliage_tint: [0.68, 0.62, 0.28],
                },
                features: vec![caves_feature_id(), ores_feature_id(), cacti_feature_id()],
            })
            .expect("the Dune Desert biome definition must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
