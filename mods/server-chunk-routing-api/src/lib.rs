use bevy::prelude::*;
use server_chunk_provider_api::{ChunkProviderId, ChunkViewer};
use std::sync::Arc;
use voxel_math_api::ChunkPos;
use world_instance_api::WorldInstanceId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ServerChunkRoute {
    pub instance: WorldInstanceId,
    pub provider: ChunkProviderId,
}

#[derive(Resource, Clone)]
pub struct ServerChunkRouter {
    route: Arc<dyn Fn(ChunkViewer, ChunkPos) -> Option<ServerChunkRoute> + Send + Sync>,
}

impl ServerChunkRouter {
    pub fn new<R>(route: R) -> Self
    where
        R: Fn(ChunkViewer, ChunkPos) -> Option<ServerChunkRoute> + Send + Sync + 'static,
    {
        Self {
            route: Arc::new(route),
        }
    }

    pub fn route(&self, viewer: ChunkViewer, position: ChunkPos) -> Option<ServerChunkRoute> {
        (self.route)(viewer, position)
    }
}

pub trait ServerChunkRoutingApi: Send + Sync + 'static {}
