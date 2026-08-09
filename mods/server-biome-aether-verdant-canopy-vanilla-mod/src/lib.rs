use bevy_mod::BevyMod;
use generated_biome_registry::BiomeId;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeClimate, BiomeDefinition, BiomeTerrain, BiomeVisuals, Dimension, ServerBiomeApi,
    ServerBiomeRegistry,
};
use server_biome_feature_aether_lights_vanilla_mod::{
    ServerBiomeFeatureAetherLightsVanillaMod, verdant_lights_feature_id,
};
use server_biome_feature_cherry_trees_vanilla_mod::{
    ServerBiomeFeatureCherryTreesVanillaMod, dense_cherry_trees_feature_id,
};
use server_biome_feature_pale_oak_trees_vanilla_mod::{
    ServerBiomeFeaturePaleOakTreesVanillaMod, pale_oak_trees_feature_id,
};
use server_biome_feature_surface_patches_vanilla_mod::{
    ServerBiomeFeatureSurfacePatchesVanillaMod, pale_moss_patches_feature_id,
};
use tokio::task::JoinHandle;

pub struct ServerBiomeAetherVerdantCanopyVanillaMod;
impl ServerBiomeAetherVerdantCanopyVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_aether_verdant_canopy::BiomeAetherVerdantCanopyMod,
        _cherry_trees: &mut server_biome_feature_cherry_trees_vanilla_mod::ServerBiomeFeatureCherryTreesVanillaMod,
        _pale_oak_trees: &mut server_biome_feature_pale_oak_trees_vanilla_mod::ServerBiomeFeaturePaleOakTreesVanillaMod,
        _aether_lights: &mut server_biome_feature_aether_lights_vanilla_mod::ServerBiomeFeatureAetherLightsVanillaMod,
        _surface_patches: &mut server_biome_feature_surface_patches_vanilla_mod::ServerBiomeFeatureSurfacePatchesVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::AetherVerdantCanopy,
                dimension: Dimension::Aether,
                name: "Verdant Canopy",
                climate: BiomeClimate {
                    temperature: 0.64,
                    humidity: 0.94,
                    continentalness: 0.30,
                    has_precipitation: true,
                    downfall: 0.88,
                },
                terrain: BiomeTerrain {
                    base_height: 8.80,
                    height_variation: 2.80,
                    detail_variation: 0.90,
                    surface: BlockId::PaleMossBlock,
                    subsurface: BlockId::RootedDirt,
                    underground: BlockId::Calcite,
                    subsurface_depth: 5,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.34, 0.72, 0.94],
                    fog_color: [0.68, 0.86, 0.76],
                    water_color: [0.28, 0.64, 0.88],
                    grass_tint: [0.42, 0.82, 0.40],
                    foliage_tint: [0.38, 0.76, 0.36],
                },
                features: vec![
                    dense_cherry_trees_feature_id(),
                    pale_oak_trees_feature_id(),
                    verdant_lights_feature_id(),
                    pale_moss_patches_feature_id(),
                ],
            })
            .expect("the Verdant Canopy biome definition must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
