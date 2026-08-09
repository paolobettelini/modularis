use bevy_mod::BevyMod;
use generated_biome_registry::BiomeId;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeClimate, BiomeDefinition, BiomeTerrain, BiomeVisuals, Dimension, ServerBiomeApi,
    ServerBiomeRegistry,
};
use server_biome_feature_aether_lights_vanilla_mod::{
    ServerBiomeFeatureAetherLightsVanillaMod, tempest_lights_feature_id,
};
use server_biome_feature_surface_patches_vanilla_mod::{
    ServerBiomeFeatureSurfacePatchesVanillaMod, rocky_patches_feature_id,
};
use tokio::task::JoinHandle;

pub struct ServerBiomeAetherTempestIslesVanillaMod;
impl ServerBiomeAetherTempestIslesVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_aether_tempest_isles::BiomeAetherTempestIslesMod,
        _aether_lights: &mut server_biome_feature_aether_lights_vanilla_mod::ServerBiomeFeatureAetherLightsVanillaMod,
        _surface_patches: &mut server_biome_feature_surface_patches_vanilla_mod::ServerBiomeFeatureSurfacePatchesVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::AetherTempestIsles,
                dimension: Dimension::Aether,
                name: "Tempest Isles",
                climate: BiomeClimate {
                    temperature: 0.30,
                    humidity: 0.76,
                    continentalness: 0.80,
                    has_precipitation: true,
                    downfall: 0.74,
                },
                terrain: BiomeTerrain {
                    base_height: 12.20,
                    height_variation: 6.80,
                    detail_variation: 2.30,
                    surface: BlockId::OxidizedCopper,
                    subsurface: BlockId::Prismarine,
                    underground: BlockId::DarkPrismarine,
                    subsurface_depth: 5,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.22, 0.46, 0.82],
                    fog_color: [0.46, 0.66, 0.78],
                    water_color: [0.18, 0.62, 0.82],
                    grass_tint: [0.28, 0.66, 0.54],
                    foliage_tint: [0.24, 0.58, 0.50],
                },
                features: vec![tempest_lights_feature_id(), rocky_patches_feature_id()],
            })
            .expect("the Tempest Isles biome definition must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
