use bevy_mod::BevyMod;
use server_chunk_provider_api::ChunkViewer;
use server_chunk_routing_api::{ServerChunkRoute, ServerChunkRouter, ServerChunkRoutingApi};
use server_dimension_api::{ServerDimensionApi, ServerDimensions};
use tokio::task::JoinHandle;

pub struct ServerChunkRoutingDimensionsMod;

impl ServerChunkRoutingDimensionsMod {
    pub fn init<D: ServerDimensionApi>(bevy: &mut BevyMod, _dimensions_api: &mut D) -> Self {
        let dimensions = bevy.app.world().resource::<ServerDimensions>().clone();
        bevy.app
            .insert_resource(ServerChunkRouter::new(move |viewer, _| {
                let definition = match viewer {
                    ChunkViewer::Server => dimensions.default_dimension(),
                    ChunkViewer::Player(player_id) => dimensions.dimension_for(player_id),
                }?;
                Some(ServerChunkRoute {
                    instance: definition.instance,
                    provider: definition.provider,
                })
            }));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerChunkRoutingApi for ServerChunkRoutingDimensionsMod {}
