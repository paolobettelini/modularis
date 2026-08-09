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
    ServerBiomeFeatureNetherFormationsVanillaMod, sulfur_spires_feature_id,
};
use server_biome_feature_nether_ores_vanilla_mod::{
    ServerBiomeFeatureNetherOresVanillaMod, nether_ores_feature_id,
};
use tokio::task::JoinHandle;

pub struct ServerBiomeSulfurWastesVanillaMod;
impl ServerBiomeSulfurWastesVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_sulfur_wastes::BiomeSulfurWastesMod,
        _caves: &mut server_biome_feature_caves_vanilla_mod::ServerBiomeFeatureCavesVanillaMod,
        _nether_ores: &mut server_biome_feature_nether_ores_vanilla_mod::ServerBiomeFeatureNetherOresVanillaMod,
        _nether_formations: &mut server_biome_feature_nether_formations_vanilla_mod::ServerBiomeFeatureNetherFormationsVanillaMod,
        _glowstone_clusters: &mut server_biome_feature_glowstone_clusters_vanilla_mod::ServerBiomeFeatureGlowstoneClustersVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::SulfurWastes,
                dimension: Dimension::Nether,
                name: "Sulfur Wastes",
                climate: BiomeClimate {
                    temperature: 0.96,
                    humidity: 0.08,
                    continentalness: 0.64,
                    has_precipitation: false,
                    downfall: 0.00,
                },
                terrain: BiomeTerrain {
                    base_height: 7.20,
                    height_variation: 5.20,
                    detail_variation: 1.90,
                    surface: BlockId::Sulfur,
                    subsurface: BlockId::Netherrack,
                    underground: BlockId::Netherrack,
                    subsurface_depth: 5,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.32, 0.16, 0.02],
                    fog_color: [0.52, 0.34, 0.06],
                    water_color: [0.42, 0.18, 0.04],
                    grass_tint: [0.58, 0.52, 0.08],
                    foliage_tint: [0.54, 0.46, 0.06],
                },
                features: vec![
                    caves_feature_id(),
                    nether_ores_feature_id(),
                    sulfur_spires_feature_id(),
                    glowstone_clusters_feature_id(),
                ],
            })
            .expect("the Sulfur Wastes biome definition must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
