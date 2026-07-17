use bevy_mod::BevyMod;
use generated_biome_registry::BiomeId;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeClimate, BiomeDefinition, BiomeTerrain, BiomeVisuals, Dimension, ServerBiomeApi,
    ServerBiomeRegistry,
};
use server_biome_feature_glowstone_clusters_vanilla_mod::{
    ServerBiomeFeatureGlowstoneClustersVanillaMod, glowstone_clusters_feature_id,
};
use server_biome_feature_oak_trees_vanilla_mod::{
    ServerBiomeFeatureOakTreesVanillaMod, dense_oak_trees_feature_id,
};
use tokio::task::JoinHandle;

pub struct ServerBiomeAetherGoldenGroveVanillaMod;

impl ServerBiomeAetherGoldenGroveVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_aether_golden_grove::BiomeAetherGoldenGroveMod,
        _oak_trees: &mut ServerBiomeFeatureOakTreesVanillaMod,
        _glowstone: &mut ServerBiomeFeatureGlowstoneClustersVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::AetherGoldenGrove,
                dimension: Dimension::Aether,
                name: "Golden Grove",
                climate: BiomeClimate {
                    temperature: 0.78,
                    humidity: 0.76,
                    continentalness: 0.44,
                    has_precipitation: true,
                    downfall: 0.62,
                },
                terrain: BiomeTerrain {
                    base_height: 10.00,
                    height_variation: 2.80,
                    detail_variation: 1.00,
                    surface: BlockId::Moss,
                    subsurface: BlockId::Dirt,
                    underground: BlockId::Calcite,
                    subsurface_depth: 4,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.48, 0.76, 1.00],
                    fog_color: [0.82, 0.90, 1.00],
                    water_color: [0.34, 0.70, 0.96],
                    grass_tint: [0.72, 0.82, 0.24],
                    foliage_tint: [0.86, 0.74, 0.20],
                },
                features: vec![
                    dense_oak_trees_feature_id(),
                    glowstone_clusters_feature_id(),
                ],
            })
            .expect("Golden Grove biome definition must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
