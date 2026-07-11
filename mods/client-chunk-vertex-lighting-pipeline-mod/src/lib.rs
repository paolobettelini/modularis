use bevy_mod::BevyMod;
use client_chunk_vertex_lighting_api::{ChunkVertexLightingPipeline, ClientChunkVertexLightingApi};
use tokio::task::JoinHandle;

pub struct ClientChunkVertexLightingPipelineMod;

impl ClientChunkVertexLightingPipelineMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app.init_resource::<ChunkVertexLightingPipeline>();
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ClientChunkVertexLightingApi for ClientChunkVertexLightingPipelineMod {}
