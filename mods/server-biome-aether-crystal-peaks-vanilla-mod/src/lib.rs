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
use server_biome_feature_crystal_spires_vanilla_mod::{
    ServerBiomeFeatureCrystalSpiresVanillaMod, crystal_spires_feature_id,
};
use tokio::task::JoinHandle;

pub struct ServerBiomeAetherCrystalPeaksVanillaMod;
impl ServerBiomeAetherCrystalPeaksVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_aether_crystal_peaks::BiomeAetherCrystalPeaksMod,
        _crystal_spires: &mut server_biome_feature_crystal_spires_vanilla_mod::ServerBiomeFeatureCrystalSpiresVanillaMod,
        _aether_lights: &mut server_biome_feature_aether_lights_vanilla_mod::ServerBiomeFeatureAetherLightsVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::AetherCrystalPeaks,
                dimension: Dimension::Aether,
                name: "Crystal Peaks",
                climate: BiomeClimate {
                    temperature: 0.24,
                    humidity: 0.42,
                    continentalness: 0.88,
                    has_precipitation: true,
                    downfall: 0.34,
                },
                terrain: BiomeTerrain {
                    base_height: 14.00,
                    height_variation: 8.80,
                    detail_variation: 3.00,
                    surface: BlockId::Calcite,
                    subsurface: BlockId::Tuff,
                    underground: BlockId::SmoothQuartz,
                    subsurface_depth: 4,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.26, 0.58, 0.98],
                    fog_color: [0.70, 0.82, 0.98],
                    water_color: [0.26, 0.56, 0.94],
                    grass_tint: [0.60, 0.70, 0.82],
                    foliage_tint: [0.58, 0.68, 0.80],
                },
                features: vec![crystal_spires_feature_id(), pearlescent_lights_feature_id()],
            })
            .expect("the Crystal Peaks biome definition must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
