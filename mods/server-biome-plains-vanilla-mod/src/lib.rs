use bevy_mod::BevyMod;
use generated_biome_registry::BiomeId;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeClimate, BiomeDefinition, BiomeTerrain, BiomeVisuals, Dimension, ServerBiomeApi,
    ServerBiomeRegistry,
};
use server_biome_feature_caves_vanilla_mod::{ServerBiomeFeatureCavesVanillaMod, caves_feature_id};
use server_biome_feature_oak_trees_vanilla_mod::{
    ServerBiomeFeatureOakTreesVanillaMod, sparse_oak_trees_feature_id,
};
use server_biome_feature_ores_vanilla_mod::{ServerBiomeFeatureOresVanillaMod, ores_feature_id};
use server_biome_feature_short_grass_vanilla_mod::{
    ServerBiomeFeatureShortGrassVanillaMod, dense_short_grass_feature_id,
};
use server_biome_feature_surface_patches_vanilla_mod::{
    ServerBiomeFeatureSurfacePatchesVanillaMod, dry_ground_feature_id,
};
use tokio::task::JoinHandle;

pub struct ServerBiomePlainsVanillaMod;
impl ServerBiomePlainsVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_plains::BiomePlainsMod,
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
                id: BiomeId::Plains,
                dimension: Dimension::Overworld,
                name: "Plains",
                climate: BiomeClimate {
                    temperature: 0.62,
                    humidity: 0.48,
                    continentalness: 0.38,
                    has_precipitation: true,
                    downfall: 0.45,
                },
                terrain: BiomeTerrain {
                    base_height: 5.40,
                    height_variation: 2.10,
                    detail_variation: 0.70,
                    surface: BlockId::Grass,
                    subsurface: BlockId::Dirt,
                    underground: BlockId::Stone,
                    subsurface_depth: 3,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.47, 0.70, 0.96],
                    fog_color: [0.72, 0.82, 0.92],
                    water_color: [0.18, 0.43, 0.72],
                    grass_tint: [0.48, 0.72, 0.31],
                    foliage_tint: [0.40, 0.66, 0.28],
                },
                features: vec![
                    caves_feature_id(),
                    ores_feature_id(),
                    sparse_oak_trees_feature_id(),
                    dense_short_grass_feature_id(),
                    dry_ground_feature_id(),
                ],
            })
            .expect("the Plains biome definition must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
