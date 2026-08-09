use bevy_mod::BevyMod;
use generated_biome_registry::BiomeId;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeClimate, BiomeDefinition, BiomeTerrain, BiomeVisuals, Dimension, ServerBiomeApi,
    ServerBiomeRegistry,
};
use server_biome_feature_caves_vanilla_mod::{ServerBiomeFeatureCavesVanillaMod, caves_feature_id};
use server_biome_feature_mushroom_groves_vanilla_mod::{
    ServerBiomeFeatureMushroomGrovesVanillaMod, mushroom_groves_feature_id,
};
use server_biome_feature_ores_vanilla_mod::{ServerBiomeFeatureOresVanillaMod, ores_feature_id};
use tokio::task::JoinHandle;

pub struct ServerBiomeMushroomFieldsVanillaMod;
impl ServerBiomeMushroomFieldsVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_mushroom_fields::BiomeMushroomFieldsMod,
        _caves: &mut server_biome_feature_caves_vanilla_mod::ServerBiomeFeatureCavesVanillaMod,
        _ores: &mut server_biome_feature_ores_vanilla_mod::ServerBiomeFeatureOresVanillaMod,
        _mushroom_groves: &mut server_biome_feature_mushroom_groves_vanilla_mod::ServerBiomeFeatureMushroomGrovesVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::MushroomFields,
                dimension: Dimension::Overworld,
                name: "Mushroom Fields",
                climate: BiomeClimate {
                    temperature: 0.66,
                    humidity: 0.88,
                    continentalness: 0.22,
                    has_precipitation: true,
                    downfall: 0.72,
                },
                terrain: BiomeTerrain {
                    base_height: 5.40,
                    height_variation: 2.20,
                    detail_variation: 0.80,
                    surface: BlockId::Mycelium,
                    subsurface: BlockId::Dirt,
                    underground: BlockId::Stone,
                    subsurface_depth: 4,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.56, 0.66, 0.86],
                    fog_color: [0.72, 0.64, 0.74],
                    water_color: [0.28, 0.46, 0.72],
                    grass_tint: [0.52, 0.46, 0.58],
                    foliage_tint: [0.46, 0.40, 0.52],
                },
                features: vec![
                    caves_feature_id(),
                    ores_feature_id(),
                    mushroom_groves_feature_id(),
                ],
            })
            .expect("the Mushroom Fields biome definition must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
