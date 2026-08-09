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
    ServerBiomeFeatureNetherFungiVanillaMod, crimson_fungi_feature_id,
};
use server_biome_feature_nether_ores_vanilla_mod::{
    ServerBiomeFeatureNetherOresVanillaMod, nether_ores_feature_id,
};
use tokio::task::JoinHandle;

pub struct ServerBiomeCrimsonForestVanillaMod;
impl ServerBiomeCrimsonForestVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_crimson_forest::BiomeCrimsonForestMod,
        _caves: &mut server_biome_feature_caves_vanilla_mod::ServerBiomeFeatureCavesVanillaMod,
        _nether_ores: &mut server_biome_feature_nether_ores_vanilla_mod::ServerBiomeFeatureNetherOresVanillaMod,
        _nether_fungi: &mut server_biome_feature_nether_fungi_vanilla_mod::ServerBiomeFeatureNetherFungiVanillaMod,
        _glowstone_clusters: &mut server_biome_feature_glowstone_clusters_vanilla_mod::ServerBiomeFeatureGlowstoneClustersVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::CrimsonForest,
                dimension: Dimension::Nether,
                name: "Crimson Forest",
                climate: BiomeClimate {
                    temperature: 0.86,
                    humidity: 0.72,
                    continentalness: 0.44,
                    has_precipitation: false,
                    downfall: 0.00,
                },
                terrain: BiomeTerrain {
                    base_height: 6.40,
                    height_variation: 3.80,
                    detail_variation: 1.40,
                    surface: BlockId::CrimsonNylium,
                    subsurface: BlockId::Netherrack,
                    underground: BlockId::Netherrack,
                    subsurface_depth: 4,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.34, 0.04, 0.06],
                    fog_color: [0.46, 0.08, 0.10],
                    water_color: [0.38, 0.08, 0.05],
                    grass_tint: [0.50, 0.08, 0.10],
                    foliage_tint: [0.48, 0.06, 0.08],
                },
                features: vec![
                    caves_feature_id(),
                    nether_ores_feature_id(),
                    crimson_fungi_feature_id(),
                    glowstone_clusters_feature_id(),
                ],
            })
            .expect("the Crimson Forest biome definition must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
