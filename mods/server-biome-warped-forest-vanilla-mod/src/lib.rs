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

pub struct ServerBiomeWarpedForestVanillaMod;

impl ServerBiomeWarpedForestVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_warped_forest::BiomeWarpedForestMod,
        _caves: &mut ServerBiomeFeatureCavesVanillaMod,
        _glowstone: &mut ServerBiomeFeatureGlowstoneClustersVanillaMod,
        _short_grass: &mut ServerBiomeFeatureShortGrassVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::WarpedForest,
                dimension: Dimension::Nether,
                name: "Warped Forest",
                climate: BiomeClimate {
                    temperature: 0.28,
                    humidity: 0.88,
                    continentalness: 0.46,
                    has_precipitation: false,
                    downfall: 0.00,
                },
                terrain: BiomeTerrain {
                    base_height: 6.50,
                    height_variation: 3.00,
                    detail_variation: 1.50,
                    surface: BlockId::WarpedNylium,
                    subsurface: BlockId::Netherrack,
                    underground: BlockId::Netherrack,
                    subsurface_depth: 4,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.02, 0.11, 0.12],
                    fog_color: [0.03, 0.20, 0.20],
                    water_color: [0.05, 0.34, 0.30],
                    grass_tint: [0.08, 0.48, 0.42],
                    foliage_tint: [0.06, 0.42, 0.38],
                },
                features: vec![
                    caves_feature_id(),
                    glowstone_clusters_feature_id(),
                    dense_short_grass_feature_id(),
                ],
            })
            .expect("Warped Forest biome definition must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
