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
use server_biome_feature_nether_fungi_vanilla_mod::{
    ServerBiomeFeatureNetherFungiVanillaMod, warped_fungi_feature_id,
};
use server_biome_feature_nether_ores_vanilla_mod::{
    ServerBiomeFeatureNetherOresVanillaMod, nether_ores_feature_id,
};
use tokio::task::JoinHandle;

pub struct ServerBiomeWarpedForestVanillaMod;
impl ServerBiomeWarpedForestVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_warped_forest::BiomeWarpedForestMod,
        _caves: &mut server_biome_feature_caves_vanilla_mod::ServerBiomeFeatureCavesVanillaMod,
        _nether_ores: &mut server_biome_feature_nether_ores_vanilla_mod::ServerBiomeFeatureNetherOresVanillaMod,
        _nether_fungi: &mut server_biome_feature_nether_fungi_vanilla_mod::ServerBiomeFeatureNetherFungiVanillaMod,
        _glowstone_clusters: &mut server_biome_feature_glowstone_clusters_vanilla_mod::ServerBiomeFeatureGlowstoneClustersVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::WarpedForest,
                dimension: Dimension::Nether,
                name: "Warped Forest",
                climate: BiomeClimate {
                    temperature: 0.62,
                    humidity: 0.84,
                    continentalness: 0.38,
                    has_precipitation: false,
                    downfall: 0.00,
                },
                terrain: BiomeTerrain {
                    base_height: 6.80,
                    height_variation: 3.50,
                    detail_variation: 1.50,
                    surface: BlockId::WarpedNylium,
                    subsurface: BlockId::Netherrack,
                    underground: BlockId::Netherrack,
                    subsurface_depth: 4,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.12, 0.20, 0.26],
                    fog_color: [0.10, 0.32, 0.34],
                    water_color: [0.10, 0.24, 0.28],
                    grass_tint: [0.12, 0.48, 0.44],
                    foliage_tint: [0.10, 0.42, 0.40],
                },
                features: vec![
                    caves_feature_id(),
                    nether_ores_feature_id(),
                    warped_fungi_feature_id(),
                    glowstone_clusters_feature_id(),
                ],
            })
            .expect("the Warped Forest biome definition must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
