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
    ServerBiomeFeatureSurfacePatchesVanillaMod, badlands_bands_feature_id,
};
use tokio::task::JoinHandle;

pub struct ServerBiomeBadlandsVanillaMod;
impl ServerBiomeBadlandsVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_badlands::BiomeBadlandsMod,
        _caves: &mut server_biome_feature_caves_vanilla_mod::ServerBiomeFeatureCavesVanillaMod,
        _ores: &mut server_biome_feature_ores_vanilla_mod::ServerBiomeFeatureOresVanillaMod,
        _boulders: &mut server_biome_feature_boulders_vanilla_mod::ServerBiomeFeatureBouldersVanillaMod,
        _surface_patches: &mut server_biome_feature_surface_patches_vanilla_mod::ServerBiomeFeatureSurfacePatchesVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::Badlands,
                dimension: Dimension::Overworld,
                name: "Painted Badlands",
                climate: BiomeClimate {
                    temperature: 0.88,
                    humidity: 0.10,
                    continentalness: 0.76,
                    has_precipitation: false,
                    downfall: 0.02,
                },
                terrain: BiomeTerrain {
                    base_height: 9.00,
                    height_variation: 7.20,
                    detail_variation: 2.40,
                    surface: BlockId::RedSand,
                    subsurface: BlockId::OrangeTerracotta,
                    underground: BlockId::RedSandstone,
                    subsurface_depth: 7,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.66, 0.76, 0.94],
                    fog_color: [0.90, 0.58, 0.36],
                    water_color: [0.19, 0.40, 0.65],
                    grass_tint: [0.70, 0.48, 0.24],
                    foliage_tint: [0.64, 0.42, 0.22],
                },
                features: vec![
                    caves_feature_id(),
                    ores_feature_id(),
                    boulders_feature_id(),
                    badlands_bands_feature_id(),
                ],
            })
            .expect("the Painted Badlands biome definition must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
