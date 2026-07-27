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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChunkSectionError {
    EmptyPalette,
    DuplicatePaletteEntry,
    InvalidPackedData,
    PaletteIndexOutOfBounds,
}

impl ChunkSection {
    pub fn filled(block: impl Into<BlockInstance>) -> Self {
        let block = block.into();
        Self {
            palette: vec![block.clone()],
            reverse_map: HashMap::from([(block, 0)]),
            entries: PackedBitArray::filled(CHUNK_VOLUME, bits_required(1), 0),
        }
    }

    pub fn from_parts(
        palette: Vec<BlockInstance>,
        bits_per_entry: u8,
        data: Vec<u64>,
    ) -> Result<Self, ChunkSectionError> {
        if palette.is_empty() {
            return Err(ChunkSectionError::EmptyPalette);
        }
        if bits_per_entry < bits_required(palette.len()) {
            return Err(ChunkSectionError::InvalidPackedData);
        }
        let entries = PackedBitArray::from_parts(CHUNK_VOLUME, bits_per_entry, data)
            .map_err(|_| ChunkSectionError::InvalidPackedData)?;
        for index in 0..entries.len() {
            if entries.get(index) as usize >= palette.len() {
                return Err(ChunkSectionError::PaletteIndexOutOfBounds);
            }
        }
        let mut reverse_map = HashMap::with_capacity(palette.len());
        for (index, block) in palette.iter().cloned().enumerate() {
            if reverse_map.insert(block, index as u32).is_some() {
                return Err(ChunkSectionError::DuplicatePaletteEntry);
            }
        }
        Ok(Self {
            palette,
            reverse_map,
            entries,
        })
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

    pub fn uniform_block(&self) -> Option<BlockInstance> {
        (self.palette.len() == 1).then(|| self.palette[0].clone())
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
        Self::from_parts(
            wire.palette,
            wire.entries.bits_per_entry(),
            wire.entries.data().to_vec(),
        )
        .map_err(|error| serde::de::Error::custom(format!("{error:?}")))
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

    #[test]
    fn uniform_sections_have_no_packed_words() {
        let section = ChunkSection::filled(block_instance_api::BlockId::Air);
        assert_eq!(section.bits_per_entry(), 0);
        assert!(section.data().is_empty());
        assert_eq!(
            section.uniform_block().unwrap().block,
            block_instance_api::BlockId::Air
        );
    }

    #[test]
    fn reconstructs_a_section_from_palette_and_words() {
        let mut original = ChunkSection::filled(block_instance_api::BlockId::Air);
        original.set(
            LocalBlockPos::new(3, 4, 5).unwrap(),
            block_instance_api::BlockId::Stone,
        );
        let restored = ChunkSection::from_parts(
            original.palette().to_vec(),
            original.bits_per_entry(),
            original.data().to_vec(),
        )
        .unwrap();
        assert_eq!(restored, original);
    }

    #[test]
    fn rejects_too_few_bits_for_the_palette() {
        let result = ChunkSection::from_parts(
            vec![
                block_instance_api::BlockId::Air.into(),
                block_instance_api::BlockId::Stone.into(),
                block_instance_api::BlockId::Dirt.into(),
            ],
            1,
            vec![0; 64],
        );
        assert_eq!(result, Err(ChunkSectionError::InvalidPackedData));
    }
}
