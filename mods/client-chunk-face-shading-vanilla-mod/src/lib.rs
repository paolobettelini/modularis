use bevy_mod::BevyMod;
use block_render_api::BlockFace;
use client_chunk_vertex_lighting_api::{ChunkVertexLightingPipeline, ClientChunkVertexLightingApi};
use tokio::task::JoinHandle;

pub struct ClientChunkFaceShadingVanillaMod;

impl ClientChunkFaceShadingVanillaMod {
    pub fn init<L: ClientChunkVertexLightingApi>(bevy: &mut BevyMod, _lighting: &mut L) -> Self {
        bevy.app
            .world()
            .resource::<ChunkVertexLightingPipeline>()
            .register_face_stage(face_brightness);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn face_brightness(face: BlockFace) -> f32 {
    match face {
        BlockFace::Top => 1.0,
        BlockFace::South => 0.88,
        BlockFace::East | BlockFace::West => 0.78,
        BlockFace::North => 0.68,
        BlockFace::Bottom => 0.55,
    }
}
