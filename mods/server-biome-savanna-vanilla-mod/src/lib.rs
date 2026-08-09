use bevy_mod::BevyMod;
use generated_biome_registry::BiomeId;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeClimate, BiomeDefinition, BiomeTerrain, BiomeVisuals, Dimension, ServerBiomeApi,
    ServerBiomeRegistry,
};
use server_biome_feature_acacia_trees_vanilla_mod::{
    ServerBiomeFeatureAcaciaTreesVanillaMod, dense_acacia_trees_feature_id,
};
use server_biome_feature_caves_vanilla_mod::{ServerBiomeFeatureCavesVanillaMod, caves_feature_id};
use server_biome_feature_ores_vanilla_mod::{ServerBiomeFeatureOresVanillaMod, ores_feature_id};
use server_biome_feature_short_grass_vanilla_mod::{
    ServerBiomeFeatureShortGrassVanillaMod, sparse_short_grass_feature_id,
};
use server_biome_feature_surface_patches_vanilla_mod::{
    ServerBiomeFeatureSurfacePatchesVanillaMod, dry_ground_feature_id,
};
use tokio::task::JoinHandle;

pub struct ServerBiomeSavannaVanillaMod;
impl ServerBiomeSavannaVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_savanna::BiomeSavannaMod,
        _caves: &mut server_biome_feature_caves_vanilla_mod::ServerBiomeFeatureCavesVanillaMod,
        _ores: &mut server_biome_feature_ores_vanilla_mod::ServerBiomeFeatureOresVanillaMod,
        _acacia_trees: &mut server_biome_feature_acacia_trees_vanilla_mod::ServerBiomeFeatureAcaciaTreesVanillaMod,
        _short_grass: &mut server_biome_feature_short_grass_vanilla_mod::ServerBiomeFeatureShortGrassVanillaMod,
        _surface_patches: &mut server_biome_feature_surface_patches_vanilla_mod::ServerBiomeFeatureSurfacePatchesVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::Savanna,
                dimension: Dimension::Overworld,
                name: "Acacia Savanna",
                climate: BiomeClimate {
                    temperature: 0.82,
                    humidity: 0.22,
                    continentalness: 0.42,
                    has_precipitation: true,
                    downfall: 0.18,
                },
                terrain: BiomeTerrain {
                    base_height: 6.20,
                    height_variation: 3.30,
                    detail_variation: 1.10,
                    surface: BlockId::Grass,
                    subsurface: BlockId::CoarseDirt,
                    underground: BlockId::Stone,
                    subsurface_depth: 4,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.60, 0.76, 0.96],
                    fog_color: [0.80, 0.78, 0.62],
                    water_color: [0.18, 0.42, 0.68],
                    grass_tint: [0.68, 0.67, 0.28],
                    foliage_tint: [0.58, 0.60, 0.24],
                },
                features: vec![
                    caves_feature_id(),
                    ores_feature_id(),
                    dense_acacia_trees_feature_id(),
                    sparse_short_grass_feature_id(),
                    dry_ground_feature_id(),
                ],
            })
            .expect("the Acacia Savanna biome definition must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
