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
use tokio::task::JoinHandle;

pub struct ServerBiomeNetherWastesVanillaMod;

impl ServerBiomeNetherWastesVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_nether_wastes::BiomeNetherWastesMod,
        _caves: &mut ServerBiomeFeatureCavesVanillaMod,
        _glowstone: &mut ServerBiomeFeatureGlowstoneClustersVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::NetherWastes,
                dimension: Dimension::Nether,
                name: "Nether Wastes",
                climate: BiomeClimate {
                    temperature: 0.65,
                    humidity: 0.35,
                    continentalness: 0.50,
                    has_precipitation: false,
                    downfall: 0.00,
                },
                terrain: BiomeTerrain {
                    base_height: 5.00,
                    height_variation: 3.00,
                    detail_variation: 1.20,
                    surface: BlockId::Netherrack,
                    subsurface: BlockId::Netherrack,
                    underground: BlockId::Netherrack,
                    subsurface_depth: 4,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.18, 0.02, 0.03],
                    fog_color: [0.26, 0.04, 0.03],
                    water_color: [0.42, 0.08, 0.03],
                    grass_tint: [0.40, 0.12, 0.08],
                    foliage_tint: [0.36, 0.10, 0.08],
                },
                features: vec![caves_feature_id(), glowstone_clusters_feature_id()],
            })
            .expect("Nether Wastes biome definition must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
