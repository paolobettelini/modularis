use bevy::prelude::*;
use std::collections::HashMap;
use voxel_math_api::{BlockPos, ChunkPos};

#[derive(Resource, Default)]
pub struct ClientBlockDamage(HashMap<BlockPos, u8>);
impl ClientBlockDamage {
    pub fn stage(&self, position: BlockPos) -> u8 { self.0.get(&position).copied().unwrap_or(0) }
    pub fn set(&mut self, position: BlockPos, stage: u8) {
        if stage == 0 { self.0.remove(&position); } else { self.0.insert(position, stage); }
    }
    pub fn remove_chunk(&mut self, chunk: ChunkPos) -> Vec<BlockPos> {
        let removed = self.0.keys().copied().filter(|position| position.chunk() == chunk).collect::<Vec<_>>();
        for position in &removed { self.0.remove(position); }
        removed
    }
    pub fn in_chunk(&self, chunk: ChunkPos) -> Vec<(BlockPos, u8)> {
        self.0.iter().filter(|(position, _)| position.chunk() == chunk).map(|(position, stage)| (*position, *stage)).collect()
    }
    pub fn clear(&mut self) { self.0.clear(); }
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetClientBlockDamage { pub position: BlockPos, pub stage: u8 }

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClientBlockDamageChanged { pub position: BlockPos, pub stage: u8 }

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClientBlockDamageSet { Receive, Apply, Draw }

pub trait ClientBlockDamageApi: Send + Sync + 'static {}

