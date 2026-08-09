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
use server_biome_feature_short_grass_vanilla_mod::{
    ServerBiomeFeatureShortGrassVanillaMod, dense_short_grass_feature_id,
};
use server_biome_feature_spruce_trees_vanilla_mod::{
    ServerBiomeFeatureSpruceTreesVanillaMod, sparse_spruce_trees_feature_id,
};
use tokio::task::JoinHandle;

pub struct ServerBiomeAetherHighlandsVanillaMod;
impl ServerBiomeAetherHighlandsVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_aether_highlands::BiomeAetherHighlandsMod,
        _oak_trees: &mut server_biome_feature_oak_trees_vanilla_mod::ServerBiomeFeatureOakTreesVanillaMod,
        _spruce_trees: &mut server_biome_feature_spruce_trees_vanilla_mod::ServerBiomeFeatureSpruceTreesVanillaMod,
        _short_grass: &mut server_biome_feature_short_grass_vanilla_mod::ServerBiomeFeatureShortGrassVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::AetherHighlands,
                dimension: Dimension::Aether,
                name: "Aether Highlands",
                climate: BiomeClimate {
                    temperature: 0.46,
                    humidity: 0.56,
                    continentalness: 0.48,
                    has_precipitation: true,
                    downfall: 0.48,
                },
                terrain: BiomeTerrain {
                    base_height: 9.50,
                    height_variation: 4.00,
                    detail_variation: 1.20,
                    surface: BlockId::Grass,
                    subsurface: BlockId::RootedDirt,
                    underground: BlockId::Andesite,
                    subsurface_depth: 4,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.30, 0.62, 0.98],
                    fog_color: [0.72, 0.84, 0.98],
                    water_color: [0.30, 0.60, 0.94],
                    grass_tint: [0.50, 0.76, 0.38],
                    foliage_tint: [0.42, 0.70, 0.34],
                },
                features: vec![
                    sparse_oak_trees_feature_id(),
                    sparse_spruce_trees_feature_id(),
                    dense_short_grass_feature_id(),
                ],
            })
            .expect("the Aether Highlands biome definition must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
