use bevy_mod::BevyMod;
use generated_biome_registry::BiomeId;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeClimate, BiomeDefinition, BiomeTerrain, BiomeVisuals, Dimension, ServerBiomeApi,
    ServerBiomeRegistry,
};
use server_biome_feature_caves_vanilla_mod::{ServerBiomeFeatureCavesVanillaMod, caves_feature_id};
use server_biome_feature_nether_formations_vanilla_mod::{
    ServerBiomeFeatureNetherFormationsVanillaMod, blackstone_outcrops_feature_id,
};
use server_biome_feature_nether_ores_vanilla_mod::{
    ServerBiomeFeatureNetherOresVanillaMod, nether_ores_feature_id,
};
use tokio::task::JoinHandle;

pub struct ServerBiomeBlackstoneRidgesVanillaMod;
impl ServerBiomeBlackstoneRidgesVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_blackstone_ridges::BiomeBlackstoneRidgesMod,
        _caves: &mut server_biome_feature_caves_vanilla_mod::ServerBiomeFeatureCavesVanillaMod,
        _nether_ores: &mut server_biome_feature_nether_ores_vanilla_mod::ServerBiomeFeatureNetherOresVanillaMod,
        _nether_formations: &mut server_biome_feature_nether_formations_vanilla_mod::ServerBiomeFeatureNetherFormationsVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::BlackstoneRidges,
                dimension: Dimension::Nether,
                name: "Blackstone Ridges",
                climate: BiomeClimate {
                    temperature: 0.76,
                    humidity: 0.16,
                    continentalness: 0.92,
                    has_precipitation: false,
                    downfall: 0.00,
                },
                terrain: BiomeTerrain {
                    base_height: 11.50,
                    height_variation: 9.00,
                    detail_variation: 3.20,
                    surface: BlockId::PolishedBlackstone,
                    subsurface: BlockId::Blackstone,
                    underground: BlockId::Netherrack,
                    subsurface_depth: 7,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.16, 0.12, 0.14],
                    fog_color: [0.26, 0.22, 0.24],
                    water_color: [0.22, 0.10, 0.08],
                    grass_tint: [0.28, 0.26, 0.28],
                    foliage_tint: [0.24, 0.22, 0.24],
                },
                features: vec![
                    caves_feature_id(),
                    nether_ores_feature_id(),
                    blackstone_outcrops_feature_id(),
                ],
            })
            .expect("the Blackstone Ridges biome definition must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
