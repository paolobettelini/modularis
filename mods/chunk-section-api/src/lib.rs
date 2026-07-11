use block_instance_api::BlockInstance;
use packed_bit_array_api::{PackedBitArray, bits_required};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use voxel_math_api::{CHUNK_VOLUME, LocalBlockPos};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkSection {
    palette: Vec<BlockInstance>,
    reverse_map: HashMap<BlockInstance, u32>,
    entries: PackedBitArray,
}

impl ChunkSection {
    pub fn filled(block: impl Into<BlockInstance>) -> Self {
        let block = block.into();
        Self {
            palette: vec![block.clone()],
            reverse_map: HashMap::from([(block, 0)]),
            entries: PackedBitArray::filled(CHUNK_VOLUME, 1, 0),
        }
    }

    pub fn get(&self, local: LocalBlockPos) -> BlockInstance {
        self.palette[self.entries.get(local.index()) as usize].clone()
    }

    pub fn set(&mut self, local: LocalBlockPos, block: impl Into<BlockInstance>) -> BlockInstance {
        let block = block.into();
        let index = local.index();
        let previous = self.palette[self.entries.get(index) as usize].clone();
        let palette_index = match self.reverse_map.get(&block).copied() {
            Some(index) => index,
            None => {
                let index = self.palette.len() as u32;
                self.palette.push(block.clone());
                self.reverse_map.insert(block, index);
                let required = bits_required(self.palette.len());
                if required > self.entries.bits_per_entry() {
                    self.entries.repack(required);
                }
                index
            }
        };
        self.entries.set(index, palette_index);
        previous
    }

    pub fn palette(&self) -> &[BlockInstance] {
        &self.palette
    }

    pub fn reverse_map(&self) -> &HashMap<BlockInstance, u32> {
        &self.reverse_map
    }

    pub const fn bits_per_entry(&self) -> u8 {
        self.entries.bits_per_entry()
    }

    pub fn data(&self) -> &[u64] {
        self.entries.data()
    }
}

#[derive(Serialize, Deserialize)]
struct ChunkSectionWire {
    palette: Vec<BlockInstance>,
    entries: PackedBitArray,
}

impl Serialize for ChunkSection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ChunkSectionWire {
            palette: self.palette.clone(),
            entries: self.entries.clone(),
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ChunkSection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ChunkSectionWire::deserialize(deserializer)?;
        if wire.palette.is_empty() {
            return Err(serde::de::Error::custom("chunk palette cannot be empty"));
        }
        if wire.entries.len() != CHUNK_VOLUME {
            return Err(serde::de::Error::custom("invalid chunk section length"));
        }
        for index in 0..wire.entries.len() {
            if wire.entries.get(index) as usize >= wire.palette.len() {
                return Err(serde::de::Error::custom(
                    "packed palette index is out of bounds",
                ));
            }
        }
        let reverse_map = wire
            .palette
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, block)| (block, index as u32))
            .collect();
        Ok(Self {
            palette: wire.palette,
            reverse_map,
            entries: wire.entries,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grows_palette_and_repacks() {
        let mut section = ChunkSection::filled(block_instance_api::BlockId::Air);
        section.set(
            LocalBlockPos::new(0, 0, 0).unwrap(),
            block_instance_api::BlockId::Dirt,
        );
        section.set(
            LocalBlockPos::new(1, 0, 0).unwrap(),
            block_instance_api::BlockId::Stone,
        );
        assert_eq!(section.palette().len(), 3);
        assert_eq!(section.bits_per_entry(), 2);
        assert_eq!(
            section.get(LocalBlockPos::new(1, 0, 0).unwrap()).block,
            block_instance_api::BlockId::Stone
        );
        assert_eq!(section.data().len(), 128);
    }
}
