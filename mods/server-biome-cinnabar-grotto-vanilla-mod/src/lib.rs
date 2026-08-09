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
use server_biome_feature_nether_formations_vanilla_mod::{
    ServerBiomeFeatureNetherFormationsVanillaMod, cinnabar_columns_feature_id,
};
use server_biome_feature_nether_ores_vanilla_mod::{
    ServerBiomeFeatureNetherOresVanillaMod, nether_ores_feature_id,
};
use tokio::task::JoinHandle;

pub struct ServerBiomeCinnabarGrottoVanillaMod;
impl ServerBiomeCinnabarGrottoVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_cinnabar_grotto::BiomeCinnabarGrottoMod,
        _caves: &mut server_biome_feature_caves_vanilla_mod::ServerBiomeFeatureCavesVanillaMod,
        _nether_ores: &mut server_biome_feature_nether_ores_vanilla_mod::ServerBiomeFeatureNetherOresVanillaMod,
        _nether_formations: &mut server_biome_feature_nether_formations_vanilla_mod::ServerBiomeFeatureNetherFormationsVanillaMod,
        _glowstone_clusters: &mut server_biome_feature_glowstone_clusters_vanilla_mod::ServerBiomeFeatureGlowstoneClustersVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::CinnabarGrotto,
                dimension: Dimension::Nether,
                name: "Cinnabar Grotto",
                climate: BiomeClimate {
                    temperature: 0.84,
                    humidity: 0.36,
                    continentalness: 0.56,
                    has_precipitation: false,
                    downfall: 0.00,
                },
                terrain: BiomeTerrain {
                    base_height: 6.00,
                    height_variation: 4.20,
                    detail_variation: 1.60,
                    surface: BlockId::Cinnabar,
                    subsurface: BlockId::PolishedCinnabar,
                    underground: BlockId::Netherrack,
                    subsurface_depth: 5,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.30, 0.06, 0.04],
                    fog_color: [0.46, 0.12, 0.08],
                    water_color: [0.36, 0.08, 0.05],
                    grass_tint: [0.56, 0.16, 0.10],
                    foliage_tint: [0.50, 0.12, 0.08],
                },
                features: vec![
                    caves_feature_id(),
                    nether_ores_feature_id(),
                    cinnabar_columns_feature_id(),
                    glowstone_clusters_feature_id(),
                ],
            })
            .expect("the Cinnabar Grotto biome definition must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
