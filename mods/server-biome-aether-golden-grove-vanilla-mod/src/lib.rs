use bevy_mod::BevyMod;
use generated_biome_registry::BiomeId;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeClimate, BiomeDefinition, BiomeTerrain, BiomeVisuals, Dimension, ServerBiomeApi,
    ServerBiomeRegistry,
};
use server_biome_feature_aether_lights_vanilla_mod::{
    ServerBiomeFeatureAetherLightsVanillaMod, golden_lights_feature_id,
};
use server_biome_feature_glowstone_clusters_vanilla_mod::{
    ServerBiomeFeatureGlowstoneClustersVanillaMod, glowstone_clusters_feature_id,
};
use server_biome_feature_oak_trees_vanilla_mod::{
    ServerBiomeFeatureOakTreesVanillaMod, dense_oak_trees_feature_id,
};
use server_biome_feature_short_grass_vanilla_mod::{
    ServerBiomeFeatureShortGrassVanillaMod, dense_short_grass_feature_id,
};
use tokio::task::JoinHandle;

pub struct ServerBiomeAetherGoldenGroveVanillaMod;
impl ServerBiomeAetherGoldenGroveVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_aether_golden_grove::BiomeAetherGoldenGroveMod,
        _oak_trees: &mut server_biome_feature_oak_trees_vanilla_mod::ServerBiomeFeatureOakTreesVanillaMod,
        _short_grass: &mut server_biome_feature_short_grass_vanilla_mod::ServerBiomeFeatureShortGrassVanillaMod,
        _aether_lights: &mut server_biome_feature_aether_lights_vanilla_mod::ServerBiomeFeatureAetherLightsVanillaMod,
        _glowstone_clusters: &mut server_biome_feature_glowstone_clusters_vanilla_mod::ServerBiomeFeatureGlowstoneClustersVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::AetherGoldenGrove,
                dimension: Dimension::Aether,
                name: "Golden Grove",
                climate: BiomeClimate {
                    temperature: 0.72,
                    humidity: 0.74,
                    continentalness: 0.34,
                    has_precipitation: true,
                    downfall: 0.54,
                },
                terrain: BiomeTerrain {
                    base_height: 10.50,
                    height_variation: 3.20,
                    detail_variation: 1.00,
                    surface: BlockId::Moss,
                    subsurface: BlockId::Dirt,
                    underground: BlockId::SmoothQuartz,
                    subsurface_depth: 4,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.38, 0.68, 0.98],
                    fog_color: [0.84, 0.86, 0.74],
                    water_color: [0.36, 0.66, 0.96],
                    grass_tint: [0.64, 0.78, 0.34],
                    foliage_tint: [0.60, 0.72, 0.30],
                },
                features: vec![
                    dense_oak_trees_feature_id(),
                    dense_short_grass_feature_id(),
                    golden_lights_feature_id(),
                    glowstone_clusters_feature_id(),
                ],
            })
            .expect("the Golden Grove biome definition must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
