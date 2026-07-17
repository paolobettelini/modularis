use bevy_mod::BevyMod;
use generated_biome_registry::BiomeId;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeClimate, BiomeDefinition, BiomeTerrain, BiomeVisuals, Dimension, ServerBiomeApi,
    ServerBiomeRegistry,
};
use server_biome_feature_caves_vanilla_mod::{ServerBiomeFeatureCavesVanillaMod, caves_feature_id};
use server_biome_feature_ice_patches_vanilla_mod::{
    ServerBiomeFeatureIcePatchesVanillaMod, ice_patches_feature_id,
};
use server_biome_feature_ores_vanilla_mod::{ServerBiomeFeatureOresVanillaMod, ores_feature_id};
use tokio::task::JoinHandle;

pub struct ServerBiomeTundraVanillaMod;

impl ServerBiomeTundraVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_tundra::BiomeTundraMod,
        _caves: &mut ServerBiomeFeatureCavesVanillaMod,
        _ores: &mut ServerBiomeFeatureOresVanillaMod,
        _ice: &mut ServerBiomeFeatureIcePatchesVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::Tundra,
                dimension: Dimension::Overworld,
                name: "Frozen Tundra",
                climate: BiomeClimate {
                    temperature: 0.08,
                    humidity: 0.48,
                    continentalness: 0.44,
                    has_precipitation: true,
                    downfall: 0.58,
                },
                terrain: BiomeTerrain {
                    base_height: 5.0,
                    height_variation: 2.8,
                    detail_variation: 0.9,
                    surface: BlockId::Snow,
                    subsurface: BlockId::Dirt,
                    underground: BlockId::Stone,
                    subsurface_depth: 4,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.62, 0.72, 0.86],
                    fog_color: [0.78, 0.84, 0.90],
                    water_color: [0.20, 0.38, 0.62],
                    grass_tint: [0.62, 0.70, 0.66],
                    foliage_tint: [0.54, 0.66, 0.62],
                },
                features: vec![
                    caves_feature_id(),
                    ores_feature_id(),
                    ice_patches_feature_id(),
                ],
            })
            .expect("the tundra biome definition must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
