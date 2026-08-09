use bevy_mod::BevyMod;
use generated_biome_registry::BiomeId;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeClimate, BiomeDefinition, BiomeTerrain, BiomeVisuals, Dimension, ServerBiomeApi,
    ServerBiomeRegistry,
};
use server_biome_feature_boulders_vanilla_mod::{
    ServerBiomeFeatureBouldersVanillaMod, boulders_feature_id,
};
use server_biome_feature_caves_vanilla_mod::{ServerBiomeFeatureCavesVanillaMod, caves_feature_id};
use server_biome_feature_ores_vanilla_mod::{ServerBiomeFeatureOresVanillaMod, ores_feature_id};
use server_biome_feature_surface_patches_vanilla_mod::{
    ServerBiomeFeatureSurfacePatchesVanillaMod, rocky_patches_feature_id,
};
use tokio::task::JoinHandle;

pub struct ServerBiomeRockyPeaksVanillaMod;
impl ServerBiomeRockyPeaksVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_rocky_peaks::BiomeRockyPeaksMod,
        _caves: &mut server_biome_feature_caves_vanilla_mod::ServerBiomeFeatureCavesVanillaMod,
        _ores: &mut server_biome_feature_ores_vanilla_mod::ServerBiomeFeatureOresVanillaMod,
        _boulders: &mut server_biome_feature_boulders_vanilla_mod::ServerBiomeFeatureBouldersVanillaMod,
        _surface_patches: &mut server_biome_feature_surface_patches_vanilla_mod::ServerBiomeFeatureSurfacePatchesVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::RockyPeaks,
                dimension: Dimension::Overworld,
                name: "Jagged Peaks",
                climate: BiomeClimate {
                    temperature: 0.22,
                    humidity: 0.30,
                    continentalness: 0.92,
                    has_precipitation: true,
                    downfall: 0.42,
                },
                terrain: BiomeTerrain {
                    base_height: 14.00,
                    height_variation: 11.50,
                    detail_variation: 3.80,
                    surface: BlockId::Andesite,
                    subsurface: BlockId::Tuff,
                    underground: BlockId::Deepslate,
                    subsurface_depth: 4,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.44, 0.64, 0.90],
                    fog_color: [0.68, 0.74, 0.80],
                    water_color: [0.18, 0.42, 0.70],
                    grass_tint: [0.48, 0.56, 0.46],
                    foliage_tint: [0.46, 0.54, 0.44],
                },
                features: vec![
                    caves_feature_id(),
                    ores_feature_id(),
                    boulders_feature_id(),
                    rocky_patches_feature_id(),
                ],
            })
            .expect("the Jagged Peaks biome definition must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
