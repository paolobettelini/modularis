use bevy_mod::BevyMod;
use generated_biome_registry::BiomeId;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeClimate, BiomeDefinition, BiomeTerrain, BiomeVisuals, Dimension, ServerBiomeApi,
    ServerBiomeRegistry,
};
use server_biome_feature_caves_vanilla_mod::{ServerBiomeFeatureCavesVanillaMod, caves_feature_id};
use server_biome_feature_dark_oak_trees_vanilla_mod::{
    ServerBiomeFeatureDarkOakTreesVanillaMod, dark_oak_trees_feature_id,
};
use server_biome_feature_ores_vanilla_mod::{ServerBiomeFeatureOresVanillaMod, ores_feature_id};
use server_biome_feature_surface_patches_vanilla_mod::{
    ServerBiomeFeatureSurfacePatchesVanillaMod, mossy_rocks_feature_id, podzol_patches_feature_id,
};
use tokio::task::JoinHandle;

pub struct ServerBiomeDarkForestVanillaMod;
impl ServerBiomeDarkForestVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_dark_forest::BiomeDarkForestMod,
        _caves: &mut server_biome_feature_caves_vanilla_mod::ServerBiomeFeatureCavesVanillaMod,
        _ores: &mut server_biome_feature_ores_vanilla_mod::ServerBiomeFeatureOresVanillaMod,
        _dark_oak_trees: &mut server_biome_feature_dark_oak_trees_vanilla_mod::ServerBiomeFeatureDarkOakTreesVanillaMod,
        _surface_patches: &mut server_biome_feature_surface_patches_vanilla_mod::ServerBiomeFeatureSurfacePatchesVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::DarkForest,
                dimension: Dimension::Overworld,
                name: "Dark Forest",
                climate: BiomeClimate {
                    temperature: 0.48,
                    humidity: 0.92,
                    continentalness: 0.50,
                    has_precipitation: true,
                    downfall: 0.92,
                },
                terrain: BiomeTerrain {
                    base_height: 6.80,
                    height_variation: 3.00,
                    detail_variation: 1.00,
                    surface: BlockId::Grass,
                    subsurface: BlockId::Podzol,
                    underground: BlockId::Stone,
                    subsurface_depth: 5,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.32, 0.54, 0.78],
                    fog_color: [0.46, 0.58, 0.56],
                    water_color: [0.12, 0.32, 0.58],
                    grass_tint: [0.20, 0.42, 0.16],
                    foliage_tint: [0.12, 0.34, 0.12],
                },
                features: vec![
                    caves_feature_id(),
                    ores_feature_id(),
                    dark_oak_trees_feature_id(),
                    podzol_patches_feature_id(),
                    mossy_rocks_feature_id(),
                ],
            })
            .expect("the Dark Forest biome definition must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
