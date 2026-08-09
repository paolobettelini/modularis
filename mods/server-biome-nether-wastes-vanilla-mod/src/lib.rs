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
    ServerBiomeFeatureNetherFormationsVanillaMod, bone_fields_feature_id,
};
use server_biome_feature_nether_ores_vanilla_mod::{
    ServerBiomeFeatureNetherOresVanillaMod, nether_ores_feature_id,
};
use tokio::task::JoinHandle;

pub struct ServerBiomeNetherWastesVanillaMod;
impl ServerBiomeNetherWastesVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_nether_wastes::BiomeNetherWastesMod,
        _caves: &mut server_biome_feature_caves_vanilla_mod::ServerBiomeFeatureCavesVanillaMod,
        _nether_ores: &mut server_biome_feature_nether_ores_vanilla_mod::ServerBiomeFeatureNetherOresVanillaMod,
        _glowstone_clusters: &mut server_biome_feature_glowstone_clusters_vanilla_mod::ServerBiomeFeatureGlowstoneClustersVanillaMod,
        _nether_formations: &mut server_biome_feature_nether_formations_vanilla_mod::ServerBiomeFeatureNetherFormationsVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::NetherWastes,
                dimension: Dimension::Nether,
                name: "Nether Wastes",
                climate: BiomeClimate {
                    temperature: 0.94,
                    humidity: 0.12,
                    continentalness: 0.42,
                    has_precipitation: false,
                    downfall: 0.00,
                },
                terrain: BiomeTerrain {
                    base_height: 5.20,
                    height_variation: 3.50,
                    detail_variation: 1.30,
                    surface: BlockId::Netherrack,
                    subsurface: BlockId::Netherrack,
                    underground: BlockId::Netherrack,
                    subsurface_depth: 4,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.30, 0.08, 0.08],
                    fog_color: [0.42, 0.10, 0.06],
                    water_color: [0.36, 0.10, 0.06],
                    grass_tint: [0.46, 0.18, 0.10],
                    foliage_tint: [0.42, 0.14, 0.08],
                },
                features: vec![
                    caves_feature_id(),
                    nether_ores_feature_id(),
                    glowstone_clusters_feature_id(),
                    bone_fields_feature_id(),
                ],
            })
            .expect("the Nether Wastes biome definition must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
