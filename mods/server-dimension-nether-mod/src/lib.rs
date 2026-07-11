use bevy_mod::BevyMod;
use generated_dimension_registry::{Dimension, id};
use server_chunk_provider_api::ChunkProviderId;
use server_chunk_provider_nether_mod::{NETHER_PROVIDER_ID, ServerChunkProviderNetherMod};
use server_dimension_api::{DimensionDefinition, ServerDimensionApi, ServerDimensions};
use tokio::task::JoinHandle;
use world_instance_api::WorldInstanceId;

pub struct ServerDimensionNetherMod;

impl ServerDimensionNetherMod {
    pub fn init<D: ServerDimensionApi>(
        bevy: &mut BevyMod,
        _dimensions: &mut D,
        _provider: &mut ServerChunkProviderNetherMod,
        _declaration: &mut dimension_nether::DimensionNetherMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerDimensions>()
            .register(
                DimensionDefinition {
                    id: Dimension::Nether,
                    instance: WorldInstanceId::new(id(Dimension::Nether)),
                    provider: ChunkProviderId::new(NETHER_PROVIDER_ID),
                    sky_color: [0.12, 0.015, 0.02, 1.0],
                    spawn: [0.0, 2.0, 0.0],
                },
                false,
            )
            .expect("the Nether dimension id must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
