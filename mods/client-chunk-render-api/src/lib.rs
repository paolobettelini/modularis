use bevy::prelude::*;
use std::collections::HashMap;
use voxel_math_api::ChunkPos;

#[derive(Resource, Default)]
pub struct RenderedChunks {
    pub entities: HashMap<ChunkPos, Vec<Entity>>,
}

pub trait ChunkRenderApi: Send + Sync + 'static {}
