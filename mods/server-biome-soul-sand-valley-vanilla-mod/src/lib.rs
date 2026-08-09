use bevy_mod::BevyMod;
use generated_biome_registry::BiomeId;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeClimate, BiomeDefinition, BiomeTerrain, BiomeVisuals, Dimension, ServerBiomeApi,
    ServerBiomeRegistry,
};
use server_biome_feature_caves_vanilla_mod::{ServerBiomeFeatureCavesVanillaMod, caves_feature_id};
use server_biome_feature_nether_formations_vanilla_mod::{
    ServerBiomeFeatureNetherFormationsVanillaMod, bone_fields_feature_id,
};
use server_biome_feature_nether_ores_vanilla_mod::{
    ServerBiomeFeatureNetherOresVanillaMod, nether_ores_feature_id,
};
use tokio::task::JoinHandle;

pub struct ServerBiomeSoulSandValleyVanillaMod;
impl ServerBiomeSoulSandValleyVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_soul_sand_valley::BiomeSoulSandValleyMod,
        _caves: &mut server_biome_feature_caves_vanilla_mod::ServerBiomeFeatureCavesVanillaMod,
        _nether_ores: &mut server_biome_feature_nether_ores_vanilla_mod::ServerBiomeFeatureNetherOresVanillaMod,
        _nether_formations: &mut server_biome_feature_nether_formations_vanilla_mod::ServerBiomeFeatureNetherFormationsVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::SoulSandValley,
                dimension: Dimension::Nether,
                name: "Soul Sand Valley",
                climate: BiomeClimate {
                    temperature: 0.74,
                    humidity: 0.12,
                    continentalness: 0.24,
                    has_precipitation: false,
                    downfall: 0.00,
                },
                terrain: BiomeTerrain {
                    base_height: 4.00,
                    height_variation: 2.50,
                    detail_variation: 0.80,
                    surface: BlockId::SoulSand,
                    subsurface: BlockId::SoulSoil,
                    underground: BlockId::Netherrack,
                    subsurface_depth: 6,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.24, 0.10, 0.10],
                    fog_color: [0.36, 0.20, 0.16],
                    water_color: [0.32, 0.12, 0.08],
                    grass_tint: [0.38, 0.30, 0.24],
                    foliage_tint: [0.34, 0.26, 0.22],
                },
                features: vec![
                    caves_feature_id(),
                    nether_ores_feature_id(),
                    bone_fields_feature_id(),
                ],
            })
            .expect("the Soul Sand Valley biome definition must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
