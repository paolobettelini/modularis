use bevy_mod::BevyMod;
use generated_biome_registry::BiomeId;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeClimate, BiomeDefinition, BiomeTerrain, BiomeVisuals, Dimension, ServerBiomeApi,
    ServerBiomeRegistry,
};
use server_biome_feature_birch_trees_vanilla_mod::{
    ServerBiomeFeatureBirchTreesVanillaMod, birch_trees_feature_id,
};
use server_biome_feature_caves_vanilla_mod::{ServerBiomeFeatureCavesVanillaMod, caves_feature_id};
use server_biome_feature_ores_vanilla_mod::{ServerBiomeFeatureOresVanillaMod, ores_feature_id};
use server_biome_feature_short_grass_vanilla_mod::{
    ServerBiomeFeatureShortGrassVanillaMod, sparse_short_grass_feature_id,
};
use server_biome_feature_surface_patches_vanilla_mod::{
    ServerBiomeFeatureSurfacePatchesVanillaMod, azalea_shrubs_feature_id,
};
use tokio::task::JoinHandle;

pub struct ServerBiomeBirchForestVanillaMod;
impl ServerBiomeBirchForestVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_birch_forest::BiomeBirchForestMod,
        _caves: &mut server_biome_feature_caves_vanilla_mod::ServerBiomeFeatureCavesVanillaMod,
        _ores: &mut server_biome_feature_ores_vanilla_mod::ServerBiomeFeatureOresVanillaMod,
        _birch_trees: &mut server_biome_feature_birch_trees_vanilla_mod::ServerBiomeFeatureBirchTreesVanillaMod,
        _short_grass: &mut server_biome_feature_short_grass_vanilla_mod::ServerBiomeFeatureShortGrassVanillaMod,
        _surface_patches: &mut server_biome_feature_surface_patches_vanilla_mod::ServerBiomeFeatureSurfacePatchesVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::BirchForest,
                dimension: Dimension::Overworld,
                name: "Birch Forest",
                climate: BiomeClimate {
                    temperature: 0.50,
                    humidity: 0.70,
                    continentalness: 0.45,
                    has_precipitation: true,
                    downfall: 0.72,
                },
                terrain: BiomeTerrain {
                    base_height: 7.20,
                    height_variation: 3.00,
                    detail_variation: 0.90,
                    surface: BlockId::Grass,
                    subsurface: BlockId::Dirt,
                    underground: BlockId::Stone,
                    subsurface_depth: 4,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.46, 0.70, 0.94],
                    fog_color: [0.72, 0.82, 0.84],
                    water_color: [0.18, 0.44, 0.72],
                    grass_tint: [0.52, 0.72, 0.34],
                    foliage_tint: [0.44, 0.68, 0.30],
                },
                features: vec![
                    caves_feature_id(),
                    ores_feature_id(),
                    birch_trees_feature_id(),
                    sparse_short_grass_feature_id(),
                    azalea_shrubs_feature_id(),
                ],
            })
            .expect("the Birch Forest biome definition must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
