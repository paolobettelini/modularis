use bevy_mod::BevyMod;
use generated_biome_registry::BiomeId;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeClimate, BiomeDefinition, BiomeTerrain, BiomeVisuals, Dimension, ServerBiomeApi,
    ServerBiomeRegistry,
};
use server_biome_feature_crystal_spires_vanilla_mod::{
    ServerBiomeFeatureCrystalSpiresVanillaMod, crystal_spires_feature_id,
};
use server_biome_feature_short_grass_vanilla_mod::{
    ServerBiomeFeatureShortGrassVanillaMod, sparse_short_grass_feature_id,
};
use tokio::task::JoinHandle;

pub struct ServerBiomeAetherCrystalPeaksVanillaMod;

impl ServerBiomeAetherCrystalPeaksVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_aether_crystal_peaks::BiomeAetherCrystalPeaksMod,
        _crystal_spires: &mut ServerBiomeFeatureCrystalSpiresVanillaMod,
        _short_grass: &mut ServerBiomeFeatureShortGrassVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::AetherCrystalPeaks,
                dimension: Dimension::Aether,
                name: "Crystal Peaks",
                climate: BiomeClimate {
                    temperature: 0.18,
                    humidity: 0.30,
                    continentalness: 0.92,
                    has_precipitation: true,
                    downfall: 0.20,
                },
                terrain: BiomeTerrain {
                    base_height: 13.00,
                    height_variation: 7.50,
                    detail_variation: 2.80,
                    surface: BlockId::Calcite,
                    subsurface: BlockId::Stone,
                    underground: BlockId::Stone,
                    subsurface_depth: 3,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.28, 0.58, 0.96],
                    fog_color: [0.68, 0.80, 0.98],
                    water_color: [0.24, 0.54, 0.92],
                    grass_tint: [0.62, 0.72, 0.82],
                    foliage_tint: [0.58, 0.68, 0.80],
                },
                features: vec![crystal_spires_feature_id(), sparse_short_grass_feature_id()],
            })
            .expect("Crystal Peaks biome definition must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
