use bevy_mod::BevyMod;
use generated_biome_registry::BiomeId;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeClimate, BiomeDefinition, BiomeTerrain, BiomeVisuals, Dimension, ServerBiomeApi,
    ServerBiomeRegistry,
};
use server_biome_feature_caves_vanilla_mod::{ServerBiomeFeatureCavesVanillaMod, caves_feature_id};
use tokio::task::JoinHandle;

pub struct ServerBiomeSoulSandValleyVanillaMod;

impl ServerBiomeSoulSandValleyVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_soul_sand_valley::BiomeSoulSandValleyMod,
        _caves: &mut ServerBiomeFeatureCavesVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::SoulSandValley,
                dimension: Dimension::Nether,
                name: "Soul Sand Valley",
                climate: BiomeClimate {
                    temperature: 0.32,
                    humidity: 0.10,
                    continentalness: 0.42,
                    has_precipitation: false,
                    downfall: 0.00,
                },
                terrain: BiomeTerrain {
                    base_height: 4.00,
                    height_variation: 2.40,
                    detail_variation: 0.80,
                    surface: BlockId::SoulSand,
                    subsurface: BlockId::SoulSoil,
                    underground: BlockId::Netherrack,
                    subsurface_depth: 5,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.10, 0.06, 0.08],
                    fog_color: [0.22, 0.16, 0.17],
                    water_color: [0.20, 0.10, 0.06],
                    grass_tint: [0.38, 0.32, 0.28],
                    foliage_tint: [0.34, 0.30, 0.28],
                },
                features: vec![caves_feature_id()],
            })
            .expect("Soul Sand Valley biome definition must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
