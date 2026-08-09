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
use server_biome_feature_spruce_trees_vanilla_mod::{
    ServerBiomeFeatureSpruceTreesVanillaMod, sparse_spruce_trees_feature_id,
};
use tokio::task::JoinHandle;

pub struct ServerBiomeTundraVanillaMod;
impl ServerBiomeTundraVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_tundra::BiomeTundraMod,
        _caves: &mut server_biome_feature_caves_vanilla_mod::ServerBiomeFeatureCavesVanillaMod,
        _ores: &mut server_biome_feature_ores_vanilla_mod::ServerBiomeFeatureOresVanillaMod,
        _ice_patches: &mut server_biome_feature_ice_patches_vanilla_mod::ServerBiomeFeatureIcePatchesVanillaMod,
        _spruce_trees: &mut server_biome_feature_spruce_trees_vanilla_mod::ServerBiomeFeatureSpruceTreesVanillaMod,
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
                    humidity: 0.34,
                    continentalness: 0.40,
                    has_precipitation: true,
                    downfall: 0.55,
                },
                terrain: BiomeTerrain {
                    base_height: 5.00,
                    height_variation: 2.70,
                    detail_variation: 0.80,
                    surface: BlockId::Snow,
                    subsurface: BlockId::CoarseDirt,
                    underground: BlockId::Stone,
                    subsurface_depth: 4,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.54, 0.68, 0.90],
                    fog_color: [0.78, 0.84, 0.90],
                    water_color: [0.24, 0.50, 0.78],
                    grass_tint: [0.58, 0.68, 0.62],
                    foliage_tint: [0.52, 0.64, 0.58],
                },
                features: vec![
                    caves_feature_id(),
                    ores_feature_id(),
                    ice_patches_feature_id(),
                    sparse_spruce_trees_feature_id(),
                ],
            })
            .expect("the Frozen Tundra biome definition must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
