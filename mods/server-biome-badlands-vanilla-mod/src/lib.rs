use bevy_mod::BevyMod;
use generated_biome_registry::BiomeId;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeClimate, BiomeDefinition, BiomeTerrain, BiomeVisuals, Dimension, ServerBiomeApi,
    ServerBiomeRegistry,
};
use server_biome_feature_boulders_vanilla_mod::{
    ServerBiomeFeatureBouldersVanillaMod, boulders_feature_id,
};
use server_biome_feature_caves_vanilla_mod::{ServerBiomeFeatureCavesVanillaMod, caves_feature_id};
use server_biome_feature_ores_vanilla_mod::{ServerBiomeFeatureOresVanillaMod, ores_feature_id};
use tokio::task::JoinHandle;

pub struct ServerBiomeBadlandsVanillaMod;

impl ServerBiomeBadlandsVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_badlands::BiomeBadlandsMod,
        _caves: &mut ServerBiomeFeatureCavesVanillaMod,
        _ores: &mut ServerBiomeFeatureOresVanillaMod,
        _boulders: &mut ServerBiomeFeatureBouldersVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::Badlands,
                dimension: Dimension::Overworld,
                name: "Red Badlands",
                climate: BiomeClimate {
                    temperature: 0.88,
                    humidity: 0.08,
                    continentalness: 0.72,
                    has_precipitation: false,
                    downfall: 0.02,
                },
                terrain: BiomeTerrain {
                    base_height: 8.50,
                    height_variation: 6.00,
                    detail_variation: 2.00,
                    surface: BlockId::RedSand,
                    subsurface: BlockId::Terracotta,
                    underground: BlockId::Stone,
                    subsurface_depth: 6,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.64, 0.76, 0.94],
                    fog_color: [0.88, 0.62, 0.42],
                    water_color: [0.20, 0.40, 0.66],
                    grass_tint: [0.72, 0.54, 0.28],
                    foliage_tint: [0.66, 0.48, 0.24],
                },
                features: vec![caves_feature_id(), ores_feature_id(), boulders_feature_id()],
            })
            .expect("Red Badlands biome definition must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
