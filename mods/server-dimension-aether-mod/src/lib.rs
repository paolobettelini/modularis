use bevy_mod::BevyMod;
use generated_dimension_registry::{Dimension, id};
use server_chunk_provider_aether_mod::{AETHER_PROVIDER_ID, ServerChunkProviderAetherMod};
use server_chunk_provider_api::ChunkProviderId;
use server_dimension_api::{DimensionDefinition, ServerDimensionApi, ServerDimensions};
use tokio::task::JoinHandle;
use world_instance_api::WorldInstanceId;

pub struct ServerDimensionAetherMod;

impl ServerDimensionAetherMod {
    pub fn init<D: ServerDimensionApi>(
        bevy: &mut BevyMod,
        _dimensions: &mut D,
        _provider: &mut ServerChunkProviderAetherMod,
        _declaration: &mut dimension_aether::DimensionAetherMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerDimensions>()
            .register(
                DimensionDefinition {
                    id: Dimension::Aether,
                    instance: WorldInstanceId::new(id(Dimension::Aether)),
                    provider: ChunkProviderId::new(AETHER_PROVIDER_ID),
                    sky_color: [0.24, 0.55, 0.92, 1.0],
                    spawn: [0.0, 10.0, 0.0],
                },
                false,
            )
            .expect("the Aether dimension id must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
