use bevy_mod::BevyMod;
use generated_biome_registry::BiomeId;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeClimate, BiomeDefinition, BiomeTerrain, BiomeVisuals, ServerBiomeApi, ServerBiomeRegistry,
};
use server_biome_feature_boulders_vanilla_mod::{
    ServerBiomeFeatureBouldersVanillaMod, boulders_feature_id,
};
use server_biome_feature_caves_vanilla_mod::{ServerBiomeFeatureCavesVanillaMod, caves_feature_id};
use server_biome_feature_ores_vanilla_mod::{ServerBiomeFeatureOresVanillaMod, ores_feature_id};
use tokio::task::JoinHandle;

pub struct ServerBiomeRockyPeaksVanillaMod;

impl ServerBiomeRockyPeaksVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_rocky_peaks::BiomeRockyPeaksMod,
        _caves: &mut ServerBiomeFeatureCavesVanillaMod,
        _ores: &mut ServerBiomeFeatureOresVanillaMod,
        _boulders: &mut ServerBiomeFeatureBouldersVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::RockyPeaks,
                name: "Rocky Peaks",
                climate: BiomeClimate {
                    temperature: 0.32,
                    humidity: 0.28,
                    continentalness: 0.90,
                    has_precipitation: true,
                    downfall: 0.28,
                },
                terrain: BiomeTerrain {
                    base_height: 13.0,
                    height_variation: 10.0,
                    detail_variation: 3.4,
                    surface: BlockId::Stone,
                    subsurface: BlockId::Gravel,
                    underground: BlockId::Stone,
                    subsurface_depth: 3,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.42, 0.62, 0.82],
                    fog_color: [0.58, 0.66, 0.72],
                    water_color: [0.18, 0.34, 0.56],
                    grass_tint: [0.46, 0.52, 0.36],
                    foliage_tint: [0.38, 0.46, 0.32],
                },
                features: vec![caves_feature_id(), ores_feature_id(), boulders_feature_id()],
            })
            .expect("the rocky peaks biome definition must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
