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

pub struct ServerBiomeBasaltDeltasVanillaMod;

impl ServerBiomeBasaltDeltasVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_basalt_deltas::BiomeBasaltDeltasMod,
        _caves: &mut ServerBiomeFeatureCavesVanillaMod,
        _glowstone: &mut ServerBiomeFeatureGlowstoneClustersVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::BasaltDeltas,
                dimension: Dimension::Nether,
                name: "Basalt Deltas",
                climate: BiomeClimate {
                    temperature: 0.72,
                    humidity: 0.12,
                    continentalness: 0.90,
                    has_precipitation: false,
                    downfall: 0.00,
                },
                terrain: BiomeTerrain {
                    base_height: 9.00,
                    height_variation: 7.00,
                    detail_variation: 2.60,
                    surface: BlockId::Basalt,
                    subsurface: BlockId::Blackstone,
                    underground: BlockId::Netherrack,
                    subsurface_depth: 6,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.08, 0.07, 0.08],
                    fog_color: [0.20, 0.18, 0.20],
                    water_color: [0.34, 0.10, 0.04],
                    grass_tint: [0.28, 0.27, 0.28],
                    foliage_tint: [0.25, 0.24, 0.25],
                },
                features: vec![caves_feature_id(), glowstone_clusters_feature_id()],
            })
            .expect("Basalt Deltas biome definition must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
