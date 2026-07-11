use bevy_mod::BevyMod;
use server_chunk_provider_api::ChunkProviderId;
use server_chunk_routing_api::{ServerChunkRoute, ServerChunkRouter, ServerChunkRoutingApi};
use tokio::task::JoinHandle;
use world_instance_api::WorldInstanceId;

pub struct ServerChunkRoutingSingleWorldMod;

impl ServerChunkRoutingSingleWorldMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app.insert_resource(ServerChunkRouter::new(|_, _| {
            Some(ServerChunkRoute {
                instance: WorldInstanceId::new("demo:overworld"),
                provider: ChunkProviderId::primary(),
            })
        }));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerChunkRoutingApi for ServerChunkRoutingSingleWorldMod {}
