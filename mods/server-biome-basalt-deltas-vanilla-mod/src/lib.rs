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
    ServerBiomeFeatureNetherFormationsVanillaMod, blackstone_outcrops_feature_id,
};
use server_biome_feature_nether_ores_vanilla_mod::{
    ServerBiomeFeatureNetherOresVanillaMod, nether_ores_feature_id,
};
use tokio::task::JoinHandle;

pub struct ServerBiomeBasaltDeltasVanillaMod;
impl ServerBiomeBasaltDeltasVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_basalt_deltas::BiomeBasaltDeltasMod,
        _caves: &mut server_biome_feature_caves_vanilla_mod::ServerBiomeFeatureCavesVanillaMod,
        _nether_ores: &mut server_biome_feature_nether_ores_vanilla_mod::ServerBiomeFeatureNetherOresVanillaMod,
        _nether_formations: &mut server_biome_feature_nether_formations_vanilla_mod::ServerBiomeFeatureNetherFormationsVanillaMod,
        _glowstone_clusters: &mut server_biome_feature_glowstone_clusters_vanilla_mod::ServerBiomeFeatureGlowstoneClustersVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::BasaltDeltas,
                dimension: Dimension::Nether,
                name: "Basalt Deltas",
                climate: BiomeClimate {
                    temperature: 0.90,
                    humidity: 0.24,
                    continentalness: 0.78,
                    has_precipitation: false,
                    downfall: 0.00,
                },
                terrain: BiomeTerrain {
                    base_height: 9.80,
                    height_variation: 8.00,
                    detail_variation: 2.80,
                    surface: BlockId::PolishedBasalt,
                    subsurface: BlockId::Blackstone,
                    underground: BlockId::Netherrack,
                    subsurface_depth: 7,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.22, 0.10, 0.10],
                    fog_color: [0.34, 0.22, 0.20],
                    water_color: [0.30, 0.10, 0.06],
                    grass_tint: [0.34, 0.30, 0.28],
                    foliage_tint: [0.30, 0.28, 0.26],
                },
                features: vec![
                    caves_feature_id(),
                    nether_ores_feature_id(),
                    blackstone_outcrops_feature_id(),
                    glowstone_clusters_feature_id(),
                ],
            })
            .expect("the Basalt Deltas biome definition must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
