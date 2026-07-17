use bevy_mod::BevyMod;
use generated_biome_registry::BiomeId;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeClimate, BiomeDefinition, BiomeTerrain, BiomeVisuals, Dimension, ServerBiomeApi,
    ServerBiomeRegistry,
};
use server_biome_feature_birch_trees_vanilla_mod::{
    ServerBiomeFeatureBirchTreesVanillaMod, birch_trees_feature_id,
};
use server_biome_feature_caves_vanilla_mod::{ServerBiomeFeatureCavesVanillaMod, caves_feature_id};
use server_biome_feature_ores_vanilla_mod::{ServerBiomeFeatureOresVanillaMod, ores_feature_id};
use tokio::task::JoinHandle;

pub struct ServerBiomeBirchForestVanillaMod;

impl ServerBiomeBirchForestVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_birch_forest::BiomeBirchForestMod,
        _caves: &mut ServerBiomeFeatureCavesVanillaMod,
        _ores: &mut ServerBiomeFeatureOresVanillaMod,
        _birch_trees: &mut ServerBiomeFeatureBirchTreesVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::BirchForest,
                dimension: Dimension::Overworld,
                name: "Birch Forest",
                climate: BiomeClimate {
                    temperature: 0.48,
                    humidity: 0.70,
                    continentalness: 0.50,
                    has_precipitation: true,
                    downfall: 0.70,
                },
                terrain: BiomeTerrain {
                    base_height: 7.00,
                    height_variation: 3.20,
                    detail_variation: 1.00,
                    surface: BlockId::Grass,
                    subsurface: BlockId::Dirt,
                    underground: BlockId::Stone,
                    subsurface_depth: 4,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.46, 0.70, 0.93],
                    fog_color: [0.68, 0.80, 0.88],
                    water_color: [0.18, 0.43, 0.72],
                    grass_tint: [0.50, 0.72, 0.34],
                    foliage_tint: [0.44, 0.70, 0.38],
                },
                features: vec![
                    caves_feature_id(),
                    ores_feature_id(),
                    birch_trees_feature_id(),
                ],
            })
            .expect("Birch Forest biome definition must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
