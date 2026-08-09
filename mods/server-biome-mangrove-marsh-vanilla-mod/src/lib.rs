use bevy_mod::BevyMod;
use generated_biome_registry::BiomeId;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeClimate, BiomeDefinition, BiomeTerrain, BiomeVisuals, Dimension, ServerBiomeApi,
    ServerBiomeRegistry,
};
use server_biome_feature_caves_vanilla_mod::{ServerBiomeFeatureCavesVanillaMod, caves_feature_id};
use server_biome_feature_mangrove_trees_vanilla_mod::{
    ServerBiomeFeatureMangroveTreesVanillaMod, mangrove_trees_feature_id,
};
use server_biome_feature_ores_vanilla_mod::{ServerBiomeFeatureOresVanillaMod, ores_feature_id};
use server_biome_feature_surface_patches_vanilla_mod::{
    ServerBiomeFeatureSurfacePatchesVanillaMod, mud_patches_feature_id,
};
use tokio::task::JoinHandle;

pub struct ServerBiomeMangroveMarshVanillaMod;
impl ServerBiomeMangroveMarshVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_mangrove_marsh::BiomeMangroveMarshMod,
        _caves: &mut server_biome_feature_caves_vanilla_mod::ServerBiomeFeatureCavesVanillaMod,
        _ores: &mut server_biome_feature_ores_vanilla_mod::ServerBiomeFeatureOresVanillaMod,
        _mangrove_trees: &mut server_biome_feature_mangrove_trees_vanilla_mod::ServerBiomeFeatureMangroveTreesVanillaMod,
        _surface_patches: &mut server_biome_feature_surface_patches_vanilla_mod::ServerBiomeFeatureSurfacePatchesVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::MangroveMarsh,
                dimension: Dimension::Overworld,
                name: "Mangrove Marsh",
                climate: BiomeClimate {
                    temperature: 0.82,
                    humidity: 0.98,
                    continentalness: 0.18,
                    has_precipitation: true,
                    downfall: 0.96,
                },
                terrain: BiomeTerrain {
                    base_height: 3.20,
                    height_variation: 1.30,
                    detail_variation: 0.50,
                    surface: BlockId::Mud,
                    subsurface: BlockId::MuddyMangroveRoots,
                    underground: BlockId::Stone,
                    subsurface_depth: 5,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.40, 0.66, 0.84],
                    fog_color: [0.50, 0.66, 0.62],
                    water_color: [0.10, 0.34, 0.42],
                    grass_tint: [0.28, 0.52, 0.24],
                    foliage_tint: [0.20, 0.46, 0.20],
                },
                features: vec![
                    caves_feature_id(),
                    ores_feature_id(),
                    mangrove_trees_feature_id(),
                    mud_patches_feature_id(),
                ],
            })
            .expect("the Mangrove Marsh biome definition must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
