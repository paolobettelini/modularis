use bevy::prelude::*;
use block_state_api::BlockState;
use chunk_api::Chunk;
use player_network_message_types::PlayerId;
use server_chunk_provider_api::{ChunkProviderId, ChunkViewer};
use std::{collections::HashSet, sync::Arc};
use voxel_math_api::ChunkPos;
use voxel_frame_api::{VoxelBlockAddress, VoxelChunkAddress, VoxelFrameId};
use voxel_frame_registry_api::VoxelFrames;
use world_instance_api::{WorldInstanceId, WorldScopeId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResidentChunkKey {
    pub instance: WorldInstanceId,
    pub provider: ChunkProviderId,
    pub frame: VoxelFrameId,
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
    pub position: VoxelBlockAddress,
    pub previous: BlockState,
    pub current: BlockState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorldEditError {
    RouteUnavailable(VoxelChunkAddress),
    ChunkUnavailable(ResidentChunkKey),
    BlockAlreadyAir(VoxelBlockAddress),
    BlockPositionOccupied(VoxelBlockAddress),
}

pub trait ServerChunkWorldBackend: Send + Sync + 'static {
    fn frames(&self) -> VoxelFrames;
    fn resident_key(&self, viewer: ChunkViewer, position: VoxelChunkAddress) -> Option<ResidentChunkKey>;
    fn chunk(&self, viewer: ChunkViewer, position: VoxelChunkAddress) -> Option<Chunk>;
    fn block(&self, viewer: ChunkViewer, position: VoxelBlockAddress) -> Option<BlockState>;
    fn set_block(
        &self,
        viewer: ChunkViewer,
        position: VoxelBlockAddress,
        block: BlockState,
    ) -> Result<BlockMutation, WorldEditError>;
    fn place_block(
        &self,
        viewer: ChunkViewer,
        position: VoxelBlockAddress,
        block: BlockState,
    ) -> Result<BlockMutation, WorldEditError>;
    fn break_block(
        &self,
        viewer: ChunkViewer,
        position: VoxelBlockAddress,
    ) -> Result<BlockMutation, WorldEditError>;
    fn retain_resident(&self, desired: &HashSet<ResidentChunkKey>);
    fn resident_keys(&self) -> Vec<ResidentChunkKey>;
    fn discard_instance(&self, _instance: &WorldInstanceId) -> usize {
        0
    }
}

#[derive(Resource, Clone)]
pub struct ServerChunkWorld(Arc<dyn ServerChunkWorldBackend>);

impl ServerChunkWorld {
    pub fn frames(&self) -> VoxelFrames { self.0.frames() }

    pub fn new<B: ServerChunkWorldBackend>(backend: B) -> Self {
        Self(Arc::new(backend))
    }

    pub fn resident_key(
        &self,
        viewer: ChunkViewer,
        position: impl Into<VoxelChunkAddress>,
    ) -> Option<ResidentChunkKey> {
        self.0.resident_key(viewer, position.into())
    }

    pub fn resident_key_for_player(
        &self,
        player_id: PlayerId,
        position: impl Into<VoxelChunkAddress>,
    ) -> Option<ResidentChunkKey> {
        self.resident_key(ChunkViewer::Player(player_id), position)
    }

    pub fn chunk_for(&self, viewer: ChunkViewer, position: impl Into<VoxelChunkAddress>) -> Option<Chunk> {
        self.0.chunk(viewer, position.into())
    }

    pub fn chunk_for_player(&self, player_id: PlayerId, position: impl Into<VoxelChunkAddress>) -> Option<Chunk> {
        self.chunk_for(ChunkViewer::Player(player_id), position)
    }

    pub fn block_for(&self, viewer: ChunkViewer, position: impl Into<VoxelBlockAddress>) -> Option<BlockState> {
        self.0.block(viewer, position.into())
    }

    pub fn block_for_player(
        &self,
        player_id: PlayerId,
        position: impl Into<VoxelBlockAddress>,
    ) -> Option<BlockState> {
        self.block_for(ChunkViewer::Player(player_id), position)
    }

    pub fn set_block_for(&self,viewer:ChunkViewer,position:impl Into<VoxelBlockAddress>,block:impl Into<BlockState>)->Result<BlockMutation,WorldEditError> {
        self.0.set_block(viewer,position.into(),block.into())
    }

    pub fn set_block_for_player(
        &self,
        player_id: PlayerId,
        position: impl Into<VoxelBlockAddress>,
        block: impl Into<BlockState>,
    ) -> Result<BlockMutation, WorldEditError> {
        self.0
            .set_block(ChunkViewer::Player(player_id), position.into(), block.into())
    }

    pub fn place_block_for_player(
        &self,
        player_id: PlayerId,
        position: impl Into<VoxelBlockAddress>,
        block: impl Into<BlockState>,
    ) -> Result<BlockMutation, WorldEditError> {
        self.0
            .place_block(ChunkViewer::Player(player_id), position.into(), block.into())
    }

    pub fn break_block_for_player(
        &self,
        player_id: PlayerId,
        position: impl Into<VoxelBlockAddress>,
    ) -> Result<BlockMutation, WorldEditError> {
        self.0.break_block(ChunkViewer::Player(player_id), position.into())
    }

    pub fn retain_resident(&self, desired: &HashSet<ResidentChunkKey>) {
        self.0.retain_resident(desired);
    }

    pub fn resident_keys(&self) -> Vec<ResidentChunkKey> {
        self.0.resident_keys()
    }

    pub fn discard_instance(&self, instance: &WorldInstanceId) -> usize {
        self.0.discard_instance(instance)
    }
}

pub trait ServerChunkWorldApi: Send + Sync + 'static {}
