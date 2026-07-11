use bevy::prelude::*;
use block_instance_api::BlockInstance;
use chunk_api::Chunk;
use player_network_message_types::PlayerId;
use server_chunk_provider_api::{ChunkProviderId, ChunkViewer};
use std::{collections::HashSet, sync::Arc};
use voxel_math_api::{BlockPos, ChunkPos};
use world_instance_api::{WorldInstanceId, WorldScopeId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResidentChunkKey {
    pub instance: WorldInstanceId,
    pub provider: ChunkProviderId,
    pub position: ChunkPos,
}

impl ResidentChunkKey {
    pub fn scope(&self) -> WorldScopeId {
        WorldScopeId::new(self.instance.clone(), self.provider.0.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockMutation {
    pub scope: WorldScopeId,
    pub position: BlockPos,
    pub previous: BlockInstance,
    pub current: BlockInstance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorldEditError {
    RouteUnavailable(ChunkPos),
    ChunkUnavailable(ResidentChunkKey),
    BlockAlreadyAir(BlockPos),
    BlockPositionOccupied(BlockPos),
}

pub trait ServerChunkWorldBackend: Send + Sync + 'static {
    fn resident_key(&self, viewer: ChunkViewer, position: ChunkPos) -> Option<ResidentChunkKey>;
    fn chunk(&self, viewer: ChunkViewer, position: ChunkPos) -> Option<Chunk>;
    fn block(&self, viewer: ChunkViewer, position: BlockPos) -> Option<BlockInstance>;
    fn set_block(
        &self,
        viewer: ChunkViewer,
        position: BlockPos,
        block: BlockInstance,
    ) -> Result<BlockMutation, WorldEditError>;
    fn place_block(
        &self,
        viewer: ChunkViewer,
        position: BlockPos,
        block: BlockInstance,
    ) -> Result<BlockMutation, WorldEditError>;
    fn break_block(
        &self,
        viewer: ChunkViewer,
        position: BlockPos,
    ) -> Result<BlockMutation, WorldEditError>;
    fn retain_resident(&self, desired: &HashSet<ResidentChunkKey>);
    fn resident_keys(&self) -> Vec<ResidentChunkKey>;
}

#[derive(Resource, Clone)]
pub struct ServerChunkWorld(Arc<dyn ServerChunkWorldBackend>);

impl ServerChunkWorld {
    pub fn new<B: ServerChunkWorldBackend>(backend: B) -> Self {
        Self(Arc::new(backend))
    }

    pub fn resident_key(
        &self,
        viewer: ChunkViewer,
        position: ChunkPos,
    ) -> Option<ResidentChunkKey> {
        self.0.resident_key(viewer, position)
    }

    pub fn resident_key_for_player(
        &self,
        player_id: PlayerId,
        position: ChunkPos,
    ) -> Option<ResidentChunkKey> {
        self.resident_key(ChunkViewer::Player(player_id), position)
    }

    pub fn chunk_for(&self, viewer: ChunkViewer, position: ChunkPos) -> Option<Chunk> {
        self.0.chunk(viewer, position)
    }

    pub fn chunk_for_player(&self, player_id: PlayerId, position: ChunkPos) -> Option<Chunk> {
        self.chunk_for(ChunkViewer::Player(player_id), position)
    }

    pub fn block_for(&self, viewer: ChunkViewer, position: BlockPos) -> Option<BlockInstance> {
        self.0.block(viewer, position)
    }

    pub fn block_for_player(
        &self,
        player_id: PlayerId,
        position: BlockPos,
    ) -> Option<BlockInstance> {
        self.block_for(ChunkViewer::Player(player_id), position)
    }

    pub fn set_block_for_player(
        &self,
        player_id: PlayerId,
        position: BlockPos,
        block: impl Into<BlockInstance>,
    ) -> Result<BlockMutation, WorldEditError> {
        self.0
            .set_block(ChunkViewer::Player(player_id), position, block.into())
    }

    pub fn place_block_for_player(
        &self,
        player_id: PlayerId,
        position: BlockPos,
        block: impl Into<BlockInstance>,
    ) -> Result<BlockMutation, WorldEditError> {
        self.0
            .place_block(ChunkViewer::Player(player_id), position, block.into())
    }

    pub fn break_block_for_player(
        &self,
        player_id: PlayerId,
        position: BlockPos,
    ) -> Result<BlockMutation, WorldEditError> {
        self.0.break_block(ChunkViewer::Player(player_id), position)
    }

    pub fn retain_resident(&self, desired: &HashSet<ResidentChunkKey>) {
        self.0.retain_resident(desired);
    }

    pub fn resident_keys(&self) -> Vec<ResidentChunkKey> {
        self.0.resident_keys()
    }
}

pub trait ServerChunkWorldApi: Send + Sync + 'static {}
