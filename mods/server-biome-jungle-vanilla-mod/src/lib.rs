use bevy_mod::BevyMod;
use generated_biome_registry::BiomeId;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeClimate, BiomeDefinition, BiomeTerrain, BiomeVisuals, Dimension, ServerBiomeApi,
    ServerBiomeRegistry,
};
use server_biome_feature_bamboo_groves_vanilla_mod::{
    ServerBiomeFeatureBambooGrovesVanillaMod, bamboo_groves_feature_id,
};
use server_biome_feature_caves_vanilla_mod::{ServerBiomeFeatureCavesVanillaMod, caves_feature_id};
use server_biome_feature_jungle_trees_vanilla_mod::{
    ServerBiomeFeatureJungleTreesVanillaMod, jungle_trees_feature_id,
};
use server_biome_feature_melon_patches_vanilla_mod::{
    ServerBiomeFeatureMelonPatchesVanillaMod, melon_patches_feature_id,
};
use server_biome_feature_ores_vanilla_mod::{ServerBiomeFeatureOresVanillaMod, ores_feature_id};
use server_biome_feature_short_grass_vanilla_mod::{
    ServerBiomeFeatureShortGrassVanillaMod, sparse_short_grass_feature_id,
};
use server_biome_feature_surface_patches_vanilla_mod::{
    ServerBiomeFeatureSurfacePatchesVanillaMod, rooted_patches_feature_id,
};
use tokio::task::JoinHandle;

pub struct ServerBiomeJungleVanillaMod;
impl ServerBiomeJungleVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_jungle::BiomeJungleMod,
        _caves: &mut server_biome_feature_caves_vanilla_mod::ServerBiomeFeatureCavesVanillaMod,
        _ores: &mut server_biome_feature_ores_vanilla_mod::ServerBiomeFeatureOresVanillaMod,
        _jungle_trees: &mut server_biome_feature_jungle_trees_vanilla_mod::ServerBiomeFeatureJungleTreesVanillaMod,
        _bamboo_groves: &mut server_biome_feature_bamboo_groves_vanilla_mod::ServerBiomeFeatureBambooGrovesVanillaMod,
        _melon_patches: &mut server_biome_feature_melon_patches_vanilla_mod::ServerBiomeFeatureMelonPatchesVanillaMod,
        _surface_patches: &mut server_biome_feature_surface_patches_vanilla_mod::ServerBiomeFeatureSurfacePatchesVanillaMod,
        _short_grass: &mut server_biome_feature_short_grass_vanilla_mod::ServerBiomeFeatureShortGrassVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::Jungle,
                dimension: Dimension::Overworld,
                name: "Emerald Jungle",
                climate: BiomeClimate {
                    temperature: 0.90,
                    humidity: 0.96,
                    continentalness: 0.46,
                    has_precipitation: true,
                    downfall: 0.96,
                },
                terrain: BiomeTerrain {
                    base_height: 7.60,
                    height_variation: 4.20,
                    detail_variation: 1.40,
                    surface: BlockId::Grass,
                    subsurface: BlockId::RootedDirt,
                    underground: BlockId::Stone,
                    subsurface_depth: 5,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.42, 0.72, 0.92],
                    fog_color: [0.52, 0.74, 0.68],
                    water_color: [0.12, 0.48, 0.66],
                    grass_tint: [0.20, 0.64, 0.18],
                    foliage_tint: [0.12, 0.54, 0.14],
                },
                features: vec![
                    caves_feature_id(),
                    ores_feature_id(),
                    jungle_trees_feature_id(),
                    bamboo_groves_feature_id(),
                    melon_patches_feature_id(),
                    rooted_patches_feature_id(),
                    sparse_short_grass_feature_id(),
                ],
            })
            .expect("the Emerald Jungle biome definition must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
