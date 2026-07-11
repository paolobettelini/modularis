use block_instance_api::BlockInstance;
use chunk_section_api::ChunkSection;
use serde::{Deserialize, Serialize};
use voxel_math_api::{CHUNK_VOLUME, ChunkPos, LocalBlockPos};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Chunk {
    position: ChunkPos,
    section: ChunkSection,
}

impl Chunk {
    pub fn filled(position: ChunkPos, block: impl Into<BlockInstance>) -> Self {
        Self {
            position,
            section: ChunkSection::filled(block),
        }
    }

    pub fn position(&self) -> ChunkPos {
        self.position
    }

    pub fn get(&self, local: LocalBlockPos) -> BlockInstance {
        self.section.get(local)
    }

    pub fn set(&mut self, local: LocalBlockPos, block: impl Into<BlockInstance>) -> BlockInstance {
        self.section.set(local, block)
    }

    pub fn section(&self) -> &ChunkSection {
        &self.section
    }

    pub fn iter(&self) -> impl Iterator<Item = (LocalBlockPos, BlockInstance)> + '_ {
        (0..CHUNK_VOLUME).map(|index| {
            let layer = 16 * 16;
            let y = index / layer;
            let remainder = index % layer;
            let z = remainder / 16;
            let x = remainder % 16;
            let local = LocalBlockPos {
                x: x as u8,
                y: y as u8,
                z: z as u8,
            };
            (local, self.get(local))
        })
    }
}
