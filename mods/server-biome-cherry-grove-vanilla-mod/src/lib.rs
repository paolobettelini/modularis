use bevy_mod::BevyMod;
use generated_biome_registry::BiomeId;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeClimate, BiomeDefinition, BiomeTerrain, BiomeVisuals, Dimension, ServerBiomeApi,
    ServerBiomeRegistry,
};
use server_biome_feature_caves_vanilla_mod::{ServerBiomeFeatureCavesVanillaMod, caves_feature_id};
use server_biome_feature_cherry_trees_vanilla_mod::{
    ServerBiomeFeatureCherryTreesVanillaMod, dense_cherry_trees_feature_id,
};
use server_biome_feature_ores_vanilla_mod::{ServerBiomeFeatureOresVanillaMod, ores_feature_id};
use server_biome_feature_short_grass_vanilla_mod::{
    ServerBiomeFeatureShortGrassVanillaMod, sparse_short_grass_feature_id,
};
use server_biome_feature_surface_patches_vanilla_mod::{
    ServerBiomeFeatureSurfacePatchesVanillaMod, azalea_shrubs_feature_id,
};
use tokio::task::JoinHandle;

pub struct ServerBiomeCherryGroveVanillaMod;
impl ServerBiomeCherryGroveVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_cherry_grove::BiomeCherryGroveMod,
        _caves: &mut server_biome_feature_caves_vanilla_mod::ServerBiomeFeatureCavesVanillaMod,
        _ores: &mut server_biome_feature_ores_vanilla_mod::ServerBiomeFeatureOresVanillaMod,
        _cherry_trees: &mut server_biome_feature_cherry_trees_vanilla_mod::ServerBiomeFeatureCherryTreesVanillaMod,
        _surface_patches: &mut server_biome_feature_surface_patches_vanilla_mod::ServerBiomeFeatureSurfacePatchesVanillaMod,
        _short_grass: &mut server_biome_feature_short_grass_vanilla_mod::ServerBiomeFeatureShortGrassVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::CherryGrove,
                dimension: Dimension::Overworld,
                name: "Cherry Grove",
                climate: BiomeClimate {
                    temperature: 0.58,
                    humidity: 0.68,
                    continentalness: 0.70,
                    has_precipitation: true,
                    downfall: 0.68,
                },
                terrain: BiomeTerrain {
                    base_height: 9.20,
                    height_variation: 5.00,
                    detail_variation: 1.50,
                    surface: BlockId::Grass,
                    subsurface: BlockId::Dirt,
                    underground: BlockId::Andesite,
                    subsurface_depth: 4,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.62, 0.76, 0.98],
                    fog_color: [0.86, 0.76, 0.86],
                    water_color: [0.26, 0.50, 0.80],
                    grass_tint: [0.54, 0.72, 0.38],
                    foliage_tint: [0.78, 0.58, 0.66],
                },
                features: vec![
                    caves_feature_id(),
                    ores_feature_id(),
                    dense_cherry_trees_feature_id(),
                    azalea_shrubs_feature_id(),
                    sparse_short_grass_feature_id(),
                ],
            })
            .expect("the Cherry Grove biome definition must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
