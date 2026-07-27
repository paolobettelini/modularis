use bevy_mod::BevyMod;
use generated_biome_registry::BiomeId;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeClimate, BiomeDefinition, BiomeTerrain, BiomeVisuals, Dimension, ServerBiomeApi,
    ServerBiomeRegistry,
};
use server_biome_feature_caves_vanilla_mod::{ServerBiomeFeatureCavesVanillaMod, caves_feature_id};
use server_biome_feature_glowstone_clusters_vanilla_mod::{
    ServerBiomeFeatureGlowstoneClustersVanillaMod, glowstone_clusters_feature_id,
};
use server_biome_feature_short_grass_vanilla_mod::{
    ServerBiomeFeatureShortGrassVanillaMod, dense_short_grass_feature_id,
};
use tokio::task::JoinHandle;

pub struct ServerBiomeCrimsonForestVanillaMod;

impl ServerBiomeCrimsonForestVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_crimson_forest::BiomeCrimsonForestMod,
        _caves: &mut ServerBiomeFeatureCavesVanillaMod,
        _glowstone: &mut ServerBiomeFeatureGlowstoneClustersVanillaMod,
        _short_grass: &mut ServerBiomeFeatureShortGrassVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::CrimsonForest,
                dimension: Dimension::Nether,
                name: "Crimson Forest",
                climate: BiomeClimate {
                    temperature: 0.82,
                    humidity: 0.78,
                    continentalness: 0.44,
                    has_precipitation: false,
                    downfall: 0.00,
                },
                terrain: BiomeTerrain {
                    base_height: 6.00,
                    height_variation: 3.50,
                    detail_variation: 1.40,
                    surface: BlockId::CrimsonNylium,
                    subsurface: BlockId::Netherrack,
                    underground: BlockId::Netherrack,
                    subsurface_depth: 4,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.22, 0.01, 0.03],
                    fog_color: [0.36, 0.02, 0.04],
                    water_color: [0.50, 0.04, 0.03],
                    grass_tint: [0.48, 0.08, 0.12],
                    foliage_tint: [0.42, 0.04, 0.10],
                },
                features: vec![
                    caves_feature_id(),
                    glowstone_clusters_feature_id(),
                    dense_short_grass_feature_id(),
                ],
            })
            .expect("Crimson Forest biome definition must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
