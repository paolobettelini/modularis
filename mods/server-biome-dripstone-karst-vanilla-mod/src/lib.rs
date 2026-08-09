use bevy_mod::BevyMod;
use generated_biome_registry::BiomeId;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeClimate, BiomeDefinition, BiomeTerrain, BiomeVisuals, Dimension, ServerBiomeApi,
    ServerBiomeRegistry,
};
use server_biome_feature_caves_vanilla_mod::{ServerBiomeFeatureCavesVanillaMod, caves_feature_id};
use server_biome_feature_dripstone_spires_vanilla_mod::{
    ServerBiomeFeatureDripstoneSpiresVanillaMod, dripstone_spires_feature_id,
};
use server_biome_feature_ores_vanilla_mod::{ServerBiomeFeatureOresVanillaMod, ores_feature_id};
use server_biome_feature_surface_patches_vanilla_mod::{
    ServerBiomeFeatureSurfacePatchesVanillaMod, rocky_patches_feature_id,
};
use tokio::task::JoinHandle;

pub struct ServerBiomeDripstoneKarstVanillaMod;
impl ServerBiomeDripstoneKarstVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _declaration: &mut biome_dripstone_karst::BiomeDripstoneKarstMod,
        _caves: &mut server_biome_feature_caves_vanilla_mod::ServerBiomeFeatureCavesVanillaMod,
        _ores: &mut server_biome_feature_ores_vanilla_mod::ServerBiomeFeatureOresVanillaMod,
        _dripstone_spires: &mut server_biome_feature_dripstone_spires_vanilla_mod::ServerBiomeFeatureDripstoneSpiresVanillaMod,
        _surface_patches: &mut server_biome_feature_surface_patches_vanilla_mod::ServerBiomeFeatureSurfacePatchesVanillaMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_biome(BiomeDefinition {
                id: BiomeId::DripstoneKarst,
                dimension: Dimension::Overworld,
                name: "Dripstone Karst",
                climate: BiomeClimate {
                    temperature: 0.46,
                    humidity: 0.38,
                    continentalness: 0.82,
                    has_precipitation: true,
                    downfall: 0.38,
                },
                terrain: BiomeTerrain {
                    base_height: 11.00,
                    height_variation: 8.20,
                    detail_variation: 2.60,
                    surface: BlockId::Tuff,
                    subsurface: BlockId::DripstoneBlock,
                    underground: BlockId::Deepslate,
                    subsurface_depth: 5,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.44, 0.60, 0.82],
                    fog_color: [0.62, 0.64, 0.64],
                    water_color: [0.18, 0.38, 0.64],
                    grass_tint: [0.44, 0.52, 0.36],
                    foliage_tint: [0.40, 0.48, 0.34],
                },
                features: vec![
                    caves_feature_id(),
                    ores_feature_id(),
                    dripstone_spires_feature_id(),
                    rocky_patches_feature_id(),
                ],
            })
            .expect("the Dripstone Karst biome definition must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
