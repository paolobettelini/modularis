use bevy_mod::BevyMod;
use generated_biome_registry::BiomeId;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeClimate, BiomeDefinition, BiomeTerrain, BiomeVisuals, Dimension, ServerBiomeApi,
    ServerBiomeRegistry,
};
use server_biome_feature_aether_lights_vanilla_mod::{
    ServerBiomeFeatureAetherLightsVanillaMod, pearlescent_lights_feature_id,
};
use server_biome_feature_cherry_trees_vanilla_mod::{
    ServerBiomeFeatureCherryTreesVanillaMod, sparse_cherry_trees_feature_id,
};
use server_biome_feature_crystal_spires_vanilla_mod::{
    ServerBiomeFeatureCrystalSpiresVanillaMod, crystal_spires_feature_id,
};
use tokio::task::JoinHandle;

pub struct ServerBiomeAetherAmethystGardensVanillaMod;
impl ServerBiomeAetherAmethystGardensVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_aether_amethyst_gardens::BiomeAetherAmethystGardensMod,
        _crystal_spires: &mut server_biome_feature_crystal_spires_vanilla_mod::ServerBiomeFeatureCrystalSpiresVanillaMod,
        _aether_lights: &mut server_biome_feature_aether_lights_vanilla_mod::ServerBiomeFeatureAetherLightsVanillaMod,
        _cherry_trees: &mut server_biome_feature_cherry_trees_vanilla_mod::ServerBiomeFeatureCherryTreesVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::AetherAmethystGardens,
                dimension: Dimension::Aether,
                name: "Amethyst Gardens",
                climate: BiomeClimate {
                    temperature: 0.50,
                    humidity: 0.70,
                    continentalness: 0.52,
                    has_precipitation: true,
                    downfall: 0.52,
                },
                terrain: BiomeTerrain {
                    base_height: 10.80,
                    height_variation: 4.60,
                    detail_variation: 1.50,
                    surface: BlockId::AmethystBlock,
                    subsurface: BlockId::Calcite,
                    underground: BlockId::SmoothQuartz,
                    subsurface_depth: 4,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.42, 0.58, 0.98],
                    fog_color: [0.78, 0.72, 0.96],
                    water_color: [0.40, 0.62, 0.98],
                    grass_tint: [0.64, 0.58, 0.82],
                    foliage_tint: [0.60, 0.54, 0.78],
                },
                features: vec![
                    crystal_spires_feature_id(),
                    pearlescent_lights_feature_id(),
                    sparse_cherry_trees_feature_id(),
                ],
            })
            .expect("the Amethyst Gardens biome definition must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
