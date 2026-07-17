use bevy_mod::BevyMod;
use generated_biome_registry::BiomeId;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeClimate, BiomeDefinition, BiomeTerrain, BiomeVisuals, Dimension, ServerBiomeApi,
    ServerBiomeRegistry,
};
use server_biome_feature_oak_trees_vanilla_mod::{
    ServerBiomeFeatureOakTreesVanillaMod, sparse_oak_trees_feature_id,
};
use tokio::task::JoinHandle;

pub struct ServerBiomeAetherHighlandsVanillaMod;

impl ServerBiomeAetherHighlandsVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_aether_highlands::BiomeAetherHighlandsMod,
        _oak_trees: &mut ServerBiomeFeatureOakTreesVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::AetherHighlands,
                dimension: Dimension::Aether,
                name: "Aether Highlands",
                climate: BiomeClimate {
                    temperature: 0.56,
                    humidity: 0.58,
                    continentalness: 0.55,
                    has_precipitation: true,
                    downfall: 0.45,
                },
                terrain: BiomeTerrain {
                    base_height: 9.00,
                    height_variation: 3.50,
                    detail_variation: 1.20,
                    surface: BlockId::Grass,
                    subsurface: BlockId::Dirt,
                    underground: BlockId::Stone,
                    subsurface_depth: 3,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.34, 0.68, 0.98],
                    fog_color: [0.72, 0.86, 0.98],
                    water_color: [0.30, 0.62, 0.92],
                    grass_tint: [0.56, 0.80, 0.36],
                    foliage_tint: [0.48, 0.76, 0.34],
                },
                features: vec![sparse_oak_trees_feature_id()],
            })
            .expect("Aether Highlands biome definition must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
