use bevy_mod::BevyMod;
use generated_biome_registry::BiomeId;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeClimate, BiomeDefinition, BiomeTerrain, BiomeVisuals, Dimension, ServerBiomeApi,
    ServerBiomeRegistry,
};
use server_biome_feature_caves_vanilla_mod::{ServerBiomeFeatureCavesVanillaMod, caves_feature_id};
use server_biome_feature_ores_vanilla_mod::{ServerBiomeFeatureOresVanillaMod, ores_feature_id};
use server_biome_feature_pale_oak_trees_vanilla_mod::{
    ServerBiomeFeaturePaleOakTreesVanillaMod, pale_oak_trees_feature_id,
};
use server_biome_feature_surface_patches_vanilla_mod::{
    ServerBiomeFeatureSurfacePatchesVanillaMod, pale_moss_patches_feature_id,
};
use tokio::task::JoinHandle;

pub struct ServerBiomePaleGardenVanillaMod;
impl ServerBiomePaleGardenVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_pale_garden::BiomePaleGardenMod,
        _caves: &mut server_biome_feature_caves_vanilla_mod::ServerBiomeFeatureCavesVanillaMod,
        _ores: &mut server_biome_feature_ores_vanilla_mod::ServerBiomeFeatureOresVanillaMod,
        _pale_oak_trees: &mut server_biome_feature_pale_oak_trees_vanilla_mod::ServerBiomeFeaturePaleOakTreesVanillaMod,
        _surface_patches: &mut server_biome_feature_surface_patches_vanilla_mod::ServerBiomeFeatureSurfacePatchesVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::PaleGarden,
                dimension: Dimension::Overworld,
                name: "Pale Garden",
                climate: BiomeClimate {
                    temperature: 0.36,
                    humidity: 0.86,
                    continentalness: 0.52,
                    has_precipitation: true,
                    downfall: 0.78,
                },
                terrain: BiomeTerrain {
                    base_height: 6.40,
                    height_variation: 2.50,
                    detail_variation: 0.90,
                    surface: BlockId::PaleMossBlock,
                    subsurface: BlockId::RootedDirt,
                    underground: BlockId::Deepslate,
                    subsurface_depth: 4,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.42, 0.50, 0.62],
                    fog_color: [0.58, 0.60, 0.62],
                    water_color: [0.20, 0.36, 0.50],
                    grass_tint: [0.54, 0.58, 0.50],
                    foliage_tint: [0.48, 0.52, 0.46],
                },
                features: vec![
                    caves_feature_id(),
                    ores_feature_id(),
                    pale_oak_trees_feature_id(),
                    pale_moss_patches_feature_id(),
                ],
            })
            .expect("the Pale Garden biome definition must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
