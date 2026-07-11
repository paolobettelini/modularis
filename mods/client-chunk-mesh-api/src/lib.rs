use bevy::prelude::*;
use block_instance_api::BlockInstance;
use chunk_api::Chunk;
use std::collections::HashMap;
use std::sync::Arc;
use voxel_math_api::{BlockPos, ChunkPos};

#[derive(Debug, Clone)]
pub struct ChunkMeshNeighborhood {
    center: Chunk,
    chunks: HashMap<ChunkPos, Chunk>,
}

impl ChunkMeshNeighborhood {
    pub fn new(center: Chunk, neighbors: impl IntoIterator<Item = Chunk>) -> Self {
        let mut chunks = neighbors
            .into_iter()
            .map(|chunk| (chunk.position(), chunk))
            .collect::<HashMap<_, _>>();
        chunks.insert(center.position(), center.clone());
        Self { center, chunks }
    }

    pub fn center(&self) -> &Chunk {
        &self.center
    }

    pub fn block(&self, position: BlockPos) -> Option<BlockInstance> {
        self.chunks
            .get(&position.chunk())
            .map(|chunk| chunk.get(position.local()))
    }
}

#[derive(Debug, Clone, Default)]
pub struct ChunkMeshPart {
    pub texture: Option<&'static str>,
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub colors: Vec<[f32; 4]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
}

impl ChunkMeshPart {
    pub fn is_empty(&self) -> bool {
        self.indices.is_empty()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ChunkMeshData {
    pub parts: Vec<ChunkMeshPart>,
}

impl ChunkMeshData {
    pub fn is_empty(&self) -> bool {
        self.parts.iter().all(ChunkMeshPart::is_empty)
    }
}

pub trait ChunkMeshApi: Send + Sync + 'static {
    fn mesh_chunk(neighborhood: &ChunkMeshNeighborhood) -> ChunkMeshData;
}

pub type ChunkMeshFunction =
    dyn Fn(&ChunkMeshNeighborhood) -> ChunkMeshData + Send + Sync + 'static;

#[derive(Resource, Clone)]
pub struct ChunkMeshService {
    pub mesh_chunk: Arc<ChunkMeshFunction>,
}

impl ChunkMeshService {
    pub fn from_api<M: ChunkMeshApi>() -> Self {
        Self {
            mesh_chunk: Arc::new(M::mesh_chunk),
        }
    }

    pub fn new(
        mesh_chunk: impl Fn(&ChunkMeshNeighborhood) -> ChunkMeshData + Send + Sync + 'static,
    ) -> Self {
        Self {
            mesh_chunk: Arc::new(mesh_chunk),
        }
    }
}
