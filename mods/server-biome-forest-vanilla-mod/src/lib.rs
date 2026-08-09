use bevy_mod::BevyMod;
use generated_biome_registry::BiomeId;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeClimate, BiomeDefinition, BiomeTerrain, BiomeVisuals, Dimension, ServerBiomeApi,
    ServerBiomeRegistry,
};
use server_biome_feature_caves_vanilla_mod::{ServerBiomeFeatureCavesVanillaMod, caves_feature_id};
use server_biome_feature_oak_trees_vanilla_mod::{
    ServerBiomeFeatureOakTreesVanillaMod, dense_oak_trees_feature_id,
};
use server_biome_feature_ores_vanilla_mod::{ServerBiomeFeatureOresVanillaMod, ores_feature_id};
use server_biome_feature_short_grass_vanilla_mod::{
    ServerBiomeFeatureShortGrassVanillaMod, sparse_short_grass_feature_id,
};
use server_biome_feature_surface_patches_vanilla_mod::{
    ServerBiomeFeatureSurfacePatchesVanillaMod, mossy_rocks_feature_id, podzol_patches_feature_id,
};
use tokio::task::JoinHandle;

pub struct ServerBiomeForestVanillaMod;
impl ServerBiomeForestVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_forest::BiomeForestMod,
        _caves: &mut server_biome_feature_caves_vanilla_mod::ServerBiomeFeatureCavesVanillaMod,
        _ores: &mut server_biome_feature_ores_vanilla_mod::ServerBiomeFeatureOresVanillaMod,
        _oak_trees: &mut server_biome_feature_oak_trees_vanilla_mod::ServerBiomeFeatureOakTreesVanillaMod,
        _short_grass: &mut server_biome_feature_short_grass_vanilla_mod::ServerBiomeFeatureShortGrassVanillaMod,
        _surface_patches: &mut server_biome_feature_surface_patches_vanilla_mod::ServerBiomeFeatureSurfacePatchesVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::Forest,
                dimension: Dimension::Overworld,
                name: "Oak Forest",
                climate: BiomeClimate {
                    temperature: 0.55,
                    humidity: 0.84,
                    continentalness: 0.48,
                    has_precipitation: true,
                    downfall: 0.84,
                },
                terrain: BiomeTerrain {
                    base_height: 6.50,
                    height_variation: 3.40,
                    detail_variation: 1.10,
                    surface: BlockId::Grass,
                    subsurface: BlockId::RootedDirt,
                    underground: BlockId::Stone,
                    subsurface_depth: 4,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.42, 0.66, 0.90],
                    fog_color: [0.62, 0.76, 0.78],
                    water_color: [0.14, 0.38, 0.66],
                    grass_tint: [0.28, 0.56, 0.20],
                    foliage_tint: [0.18, 0.48, 0.15],
                },
                features: vec![
                    caves_feature_id(),
                    ores_feature_id(),
                    dense_oak_trees_feature_id(),
                    sparse_short_grass_feature_id(),
                    podzol_patches_feature_id(),
                    mossy_rocks_feature_id(),
                ],
            })
            .expect("the Oak Forest biome definition must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
