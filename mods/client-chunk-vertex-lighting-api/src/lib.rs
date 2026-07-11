use bevy::prelude::*;
use block_render_api::BlockFace;
use std::sync::{Arc, RwLock};

pub type FaceBrightnessStage = fn(BlockFace) -> f32;
pub type AmbientOcclusionStage = fn(VertexOcclusion) -> f32;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct VertexOcclusion {
    pub side_a: bool,
    pub side_b: bool,
    pub corner: bool,
}

#[derive(Default)]
struct LightingStages {
    face: Vec<FaceBrightnessStage>,
    ambient_occlusion: Vec<AmbientOcclusionStage>,
}

#[derive(Clone, Default)]
pub struct ChunkVertexLightingSnapshot {
    face: Vec<FaceBrightnessStage>,
    ambient_occlusion: Vec<AmbientOcclusionStage>,
}

impl ChunkVertexLightingSnapshot {
    pub fn brightness(&self, face: BlockFace, occlusion: VertexOcclusion) -> f32 {
        let face_light = self
            .face
            .iter()
            .fold(1.0, |value, stage| value * stage(face).clamp(0.0, 1.0));
        self.ambient_occlusion
            .iter()
            .fold(face_light, |value, stage| {
                value * stage(occlusion).clamp(0.0, 1.0)
            })
            .clamp(0.0, 1.0)
    }
}

/// Shared, append-only mesh-lighting pipeline. Independent mods may register
/// multiplicative stages without depending on the concrete chunk mesher.
#[derive(Resource, Clone, Default)]
pub struct ChunkVertexLightingPipeline(Arc<RwLock<LightingStages>>);

impl ChunkVertexLightingPipeline {
    pub fn register_face_stage(&self, stage: FaceBrightnessStage) {
        self.0
            .write()
            .expect("lighting pipeline poisoned")
            .face
            .push(stage);
    }

    pub fn register_ambient_occlusion_stage(&self, stage: AmbientOcclusionStage) {
        self.0
            .write()
            .expect("lighting pipeline poisoned")
            .ambient_occlusion
            .push(stage);
    }

    pub fn snapshot(&self) -> ChunkVertexLightingSnapshot {
        let stages = self.0.read().expect("lighting pipeline poisoned");
        ChunkVertexLightingSnapshot {
            face: stages.face.clone(),
            ambient_occlusion: stages.ambient_occlusion.clone(),
        }
    }
}

pub trait ClientChunkVertexLightingApi: Send + Sync + 'static {}
