use bevy::prelude::*;
use chunk_api::Chunk;
use client_dimension_api::Dimension;
use client_grass_settings_api::ClientGrassSettings;
use std::sync::Arc;

#[derive(Debug, Clone, Default)]
pub struct GrassMeshData {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub colors: Vec<[f32; 4]>,
    pub indices: Vec<u32>,
    pub blade_count: usize,
}

impl GrassMeshData {
    pub fn is_empty(&self) -> bool {
        self.indices.is_empty()
    }
}

pub trait ClientGrassMeshApi: Send + Sync + 'static {}

pub type GrassMeshFunction =
    dyn Fn(&Chunk, ClientGrassSettings, f32, Dimension) -> GrassMeshData + Send + Sync + 'static;

#[derive(Resource, Clone)]
pub struct GrassMeshService {
    pub mesh_chunk: Arc<GrassMeshFunction>,
}

impl GrassMeshService {
    pub fn new(
        mesh_chunk: impl Fn(&Chunk, ClientGrassSettings, f32, Dimension) -> GrassMeshData
        + Send
        + Sync
        + 'static,
    ) -> Self {
        Self {
            mesh_chunk: Arc::new(mesh_chunk),
        }
    }
}
