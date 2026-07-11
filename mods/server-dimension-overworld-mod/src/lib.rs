use bevy_mod::BevyMod;
use generated_dimension_registry::{Dimension, id};
use server_chunk_provider_api::ChunkProviderId;
use server_dimension_api::{DimensionDefinition, ServerDimensionApi, ServerDimensions};
use tokio::task::JoinHandle;
use world_instance_api::WorldInstanceId;

pub struct ServerDimensionOverworldMod;

impl ServerDimensionOverworldMod {
    pub fn init<D: ServerDimensionApi>(
        bevy: &mut BevyMod,
        _dimensions: &mut D,
        _declaration: &mut dimension_overworld::DimensionOverworldMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerDimensions>()
            .register(
                DimensionDefinition {
                    id: Dimension::Overworld,
                    instance: WorldInstanceId::new(id(Dimension::Overworld)),
                    provider: ChunkProviderId::primary(),
                    sky_color: [0.48, 0.72, 0.94, 1.0],
                    spawn: [0.0, 2.0, 0.0],
                },
                true,
            )
            .expect("the Overworld dimension id and default must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
