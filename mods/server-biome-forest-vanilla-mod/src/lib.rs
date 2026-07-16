use bevy_mod::BevyMod;
use generated_biome_registry::BiomeId;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeClimate, BiomeDefinition, BiomeTerrain, BiomeVisuals, ServerBiomeApi, ServerBiomeRegistry,
};
use server_biome_feature_caves_vanilla_mod::{ServerBiomeFeatureCavesVanillaMod, caves_feature_id};
use server_biome_feature_oak_trees_vanilla_mod::{
    ServerBiomeFeatureOakTreesVanillaMod, dense_oak_trees_feature_id,
};
use server_biome_feature_ores_vanilla_mod::{ServerBiomeFeatureOresVanillaMod, ores_feature_id};
use tokio::task::JoinHandle;

pub struct ServerBiomeForestVanillaMod;

impl ServerBiomeForestVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_forest::BiomeForestMod,
        _caves: &mut ServerBiomeFeatureCavesVanillaMod,
        _ores: &mut ServerBiomeFeatureOresVanillaMod,
        _trees: &mut ServerBiomeFeatureOakTreesVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::Forest,
                name: "Oak Forest",
                climate: BiomeClimate {
                    temperature: 0.56,
                    humidity: 0.82,
                    continentalness: 0.48,
                    has_precipitation: true,
                    downfall: 0.82,
                },
                terrain: BiomeTerrain {
                    base_height: 6.0,
                    height_variation: 3.0,
                    detail_variation: 1.1,
                    surface: BlockId::Grass,
                    subsurface: BlockId::Dirt,
                    underground: BlockId::Stone,
                    subsurface_depth: 4,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.44, 0.68, 0.90],
                    fog_color: [0.66, 0.78, 0.82],
                    water_color: [0.16, 0.40, 0.68],
                    grass_tint: [0.30, 0.58, 0.20],
                    foliage_tint: [0.20, 0.50, 0.16],
                },
                features: vec![
                    caves_feature_id(),
                    ores_feature_id(),
                    dense_oak_trees_feature_id(),
                ],
            })
            .expect("the forest biome definition must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
