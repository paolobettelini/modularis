use block_instance_api::{BlockId, BlockInstance, all_blocks, block_id_as_str, block_id_from_str};
use chunk_api::Chunk;
use chunk_section_api::ChunkSection;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    error::Error,
    fmt,
};
use voxel_math_api::ChunkPos;

const INDEX_MAGIC: &[u8; 4] = b"PWBI";
const REGION_MAGIC: &[u8; 4] = b"PWCR";
const FORMAT_VERSION: u16 = 1;
pub const REGION_EDGE_CHUNKS: i32 = 8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorageFormatError(pub String);

impl fmt::Display for StorageFormatError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl Error for StorageFormatError {}

#[derive(Debug, Clone)]
struct BlockIndexEntry {
    id: String,
    block: Option<BlockId>,
}

#[derive(Debug, Clone)]
pub struct GlobalBlockIndex {
    entries: Vec<BlockIndexEntry>,
    reverse: HashMap<BlockId, u32>,
}

impl GlobalBlockIndex {
    pub fn current() -> Self {
        let mut ids = all_blocks()
            .iter()
            .copied()
            .map(|block| (block_id_as_str(block).to_string(), block))
            .collect::<Vec<_>>();
        ids.sort_by(|left, right| left.0.cmp(&right.0));
        Self::from_ids(ids.into_iter().map(|(id, _)| id).collect())
            .expect("generated block IDs must form a valid global index")
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, StorageFormatError> {
        let mut cursor = BinaryCursor::new(bytes);
        cursor.expect_magic(INDEX_MAGIC)?;
        expect_version(cursor.read_u16()?)?;
        let count = cursor.read_u32()? as usize;
        let mut ids = Vec::with_capacity(count);
        for _ in 0..count {
            let length = cursor.read_u16()? as usize;
            let id = std::str::from_utf8(cursor.read_bytes(length)?)
                .map_err(|error| StorageFormatError(format!("invalid block ID UTF-8: {error}")))?
                .to_string();
            ids.push(id);
        }
        cursor.finish()?;
        Self::from_ids(ids)
    }

    pub fn encode(&self) -> Result<Vec<u8>, StorageFormatError> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(INDEX_MAGIC);
        write_u16(&mut bytes, FORMAT_VERSION);
        write_u32(
            &mut bytes,
            checked_u32(self.entries.len(), "block index entries")?,
        );
        for entry in &self.entries {
            let id = entry.id.as_bytes();
            write_u16(&mut bytes, checked_u16(id.len(), "block ID length")?);
            bytes.extend_from_slice(id);
        }
        Ok(bytes)
    }

    /// Appends newly composed block IDs without changing existing indices.
    pub fn reconcile_current_blocks(&mut self) -> bool {
        let existing = self
            .entries
            .iter()
            .map(|entry| entry.id.as_str())
            .collect::<HashSet<_>>();
        let mut missing = all_blocks()
            .iter()
            .copied()
            .filter(|block| !existing.contains(block_id_as_str(*block)))
            .collect::<Vec<_>>();
        missing.sort_by_key(|block| block_id_as_str(*block));
        if missing.is_empty() {
            return false;
        }
        for block in missing {
            let index = self.entries.len() as u32;
            self.entries.push(BlockIndexEntry {
                id: block_id_as_str(block).to_string(),
                block: Some(block),
            });
            self.reverse.insert(block, index);
        }
        true
    }

    pub fn ids(&self) -> impl Iterator<Item = &str> {
        self.entries.iter().map(|entry| entry.id.as_str())
    }

    fn from_ids(ids: Vec<String>) -> Result<Self, StorageFormatError> {
        let mut seen = HashSet::new();
        let mut entries = Vec::with_capacity(ids.len());
        let mut reverse = HashMap::new();
        for id in ids {
            if !seen.insert(id.clone()) {
                return Err(StorageFormatError(format!(
                    "duplicate block ID '{id}' in global index"
                )));
            }
            let block = block_id_from_str(&id);
            if let Some(block) = block {
                reverse.insert(block, entries.len() as u32);
            }
            entries.push(BlockIndexEntry { id, block });
        }
        Ok(Self { entries, reverse })
    }

    fn index_of(&self, block: BlockId) -> Result<u32, StorageFormatError> {
        self.reverse.get(&block).copied().ok_or_else(|| {
            StorageFormatError(format!(
                "block '{}' is missing from the global index",
                block_id_as_str(block)
            ))
        })
    }

    fn block_at(&self, index: u32) -> Result<BlockId, StorageFormatError> {
        let entry = self.entries.get(index as usize).ok_or_else(|| {
            StorageFormatError(format!("global block index {index} is out of bounds"))
        })?;
        entry.block.ok_or_else(|| {
            StorageFormatError(format!(
                "saved block '{}' is not available in this composition",
                entry.id
            ))
        })
    }
}

pub fn encode_chunk(
    chunk: &Chunk,
    index: &GlobalBlockIndex,
) -> Result<Vec<u8>, StorageFormatError> {
    let section = chunk.section();
    let mut bytes = Vec::new();
    write_u16(
        &mut bytes,
        checked_u16(section.palette().len(), "chunk palette entries")?,
    );
    for instance in section.palette() {
        write_u32(&mut bytes, index.index_of(instance.block)?);
        let metadata = serde_cbor::to_vec(&instance.metadata).map_err(|error| {
            StorageFormatError(format!("failed to encode block metadata: {error}"))
        })?;
        write_u32(
            &mut bytes,
            checked_u32(metadata.len(), "block metadata length")?,
        );
        bytes.extend_from_slice(&metadata);
    }
    bytes.push(section.bits_per_entry());
    write_u32(
        &mut bytes,
        checked_u32(section.data().len(), "packed chunk words")?,
    );
    for word in section.data() {
        write_u64(&mut bytes, *word);
    }
    Ok(bytes)
}

pub fn decode_chunk(
    position: ChunkPos,
    bytes: &[u8],
    index: &GlobalBlockIndex,
) -> Result<Chunk, StorageFormatError> {
    let mut cursor = BinaryCursor::new(bytes);
    let palette_len = cursor.read_u16()? as usize;
    if palette_len == 0 {
        return Err(StorageFormatError(
            "chunk local palette cannot be empty".to_string(),
        ));
    }
    let mut palette = Vec::with_capacity(palette_len);
    for _ in 0..palette_len {
        let block = index.block_at(cursor.read_u32()?)?;
        let metadata_len = cursor.read_u32()? as usize;
        let metadata =
            serde_cbor::from_slice(cursor.read_bytes(metadata_len)?).map_err(|error| {
                StorageFormatError(format!("failed to decode block metadata: {error}"))
            })?;
        palette.push(BlockInstance { block, metadata });
    }
    let bits_per_entry = cursor.read_u8()?;
    let word_count = cursor.read_u32()? as usize;
    let mut words = Vec::with_capacity(word_count);
    for _ in 0..word_count {
        words.push(cursor.read_u64()?);
    }
    cursor.finish()?;
    let section = ChunkSection::from_parts(palette, bits_per_entry, words)
        .map_err(|error| StorageFormatError(format!("invalid chunk section: {error:?}")))?;
    Ok(Chunk::from_section(position, section))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ChunkRegionPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl ChunkRegionPos {
    pub fn from_chunk(position: ChunkPos) -> Self {
        Self {
            x: position.x.div_euclid(REGION_EDGE_CHUNKS),
            y: position.y.div_euclid(REGION_EDGE_CHUNKS),
            z: position.z.div_euclid(REGION_EDGE_CHUNKS),
        }
    }
}

pub fn encode_region(chunks: &BTreeMap<ChunkPos, Vec<u8>>) -> Result<Vec<u8>, StorageFormatError> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(REGION_MAGIC);
    write_u16(&mut bytes, FORMAT_VERSION);
    write_u32(&mut bytes, checked_u32(chunks.len(), "region chunks")?);
    for (position, payload) in chunks {
        write_i32(&mut bytes, position.x);
        write_i32(&mut bytes, position.y);
        write_i32(&mut bytes, position.z);
        write_u32(&mut bytes, checked_u32(payload.len(), "chunk payload")?);
        bytes.extend_from_slice(payload);
    }
    Ok(bytes)
}

pub fn decode_region(bytes: &[u8]) -> Result<BTreeMap<ChunkPos, Vec<u8>>, StorageFormatError> {
    let mut cursor = BinaryCursor::new(bytes);
    cursor.expect_magic(REGION_MAGIC)?;
    expect_version(cursor.read_u16()?)?;
    let count = cursor.read_u32()? as usize;
    let mut chunks = BTreeMap::new();
    for _ in 0..count {
        let position = ChunkPos::new(cursor.read_i32()?, cursor.read_i32()?, cursor.read_i32()?);
        let payload_len = cursor.read_u32()? as usize;
        let payload = cursor.read_bytes(payload_len)?.to_vec();
        if chunks.insert(position, payload).is_some() {
            return Err(StorageFormatError(format!(
                "duplicate chunk {position:?} in region"
            )));
        }
    }
    cursor.finish()?;
    Ok(chunks)
}

fn expect_version(version: u16) -> Result<(), StorageFormatError> {
    (version == FORMAT_VERSION).then_some(()).ok_or_else(|| {
        StorageFormatError(format!(
            "unsupported storage format version {version}, expected {FORMAT_VERSION}"
        ))
    })
}

fn checked_u16(value: usize, label: &str) -> Result<u16, StorageFormatError> {
    value
        .try_into()
        .map_err(|_| StorageFormatError(format!("{label} exceed u16 capacity")))
}

fn checked_u32(value: usize, label: &str) -> Result<u32, StorageFormatError> {
    value
        .try_into()
        .map_err(|_| StorageFormatError(format!("{label} exceed u32 capacity")))
}

fn write_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn write_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn write_i32(bytes: &mut Vec<u8>, value: i32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn write_u64(bytes: &mut Vec<u8>, value: u64) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

struct BinaryCursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> BinaryCursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn expect_magic(&mut self, magic: &[u8]) -> Result<(), StorageFormatError> {
        let actual = self.read_bytes(magic.len())?;
        (actual == magic)
            .then_some(())
            .ok_or_else(|| StorageFormatError("invalid binary file magic".to_string()))
    }

    fn read_u8(&mut self) -> Result<u8, StorageFormatError> {
        Ok(self.read_bytes(1)?[0])
    }

    fn read_u16(&mut self) -> Result<u16, StorageFormatError> {
        Ok(u16::from_le_bytes(self.read_array()?))
    }

    fn read_u32(&mut self) -> Result<u32, StorageFormatError> {
        Ok(u32::from_le_bytes(self.read_array()?))
    }

    fn read_i32(&mut self) -> Result<i32, StorageFormatError> {
        Ok(i32::from_le_bytes(self.read_array()?))
    }

    fn read_u64(&mut self) -> Result<u64, StorageFormatError> {
        Ok(u64::from_le_bytes(self.read_array()?))
    }

    fn read_array<const N: usize>(&mut self) -> Result<[u8; N], StorageFormatError> {
        self.read_bytes(N)?
            .try_into()
            .map_err(|_| StorageFormatError("invalid fixed-width field".to_string()))
    }

    fn read_bytes(&mut self, length: usize) -> Result<&'a [u8], StorageFormatError> {
        let end = self
            .offset
            .checked_add(length)
            .ok_or_else(|| StorageFormatError("binary offset overflow".to_string()))?;
        let value = self
            .bytes
            .get(self.offset..end)
            .ok_or_else(|| StorageFormatError("unexpected end of binary data".to_string()))?;
        self.offset = end;
        Ok(value)
    }

    fn finish(self) -> Result<(), StorageFormatError> {
        (self.offset == self.bytes.len())
            .then_some(())
            .ok_or_else(|| StorageFormatError("trailing binary data".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use voxel_math_api::LocalBlockPos;

    #[test]
    fn global_index_and_chunk_palettes_roundtrip() {
        let index = GlobalBlockIndex::current();
        let encoded_index = index.encode().unwrap();
        let restored_index = GlobalBlockIndex::decode(&encoded_index).unwrap();
        assert_eq!(
            restored_index.ids().collect::<Vec<_>>(),
            index.ids().collect::<Vec<_>>()
        );

        let position = ChunkPos::new(-2, 17, 4);
        let mut chunk = Chunk::filled(position, BlockId::Air);
        chunk.set(LocalBlockPos::new(2, 3, 4).unwrap(), BlockId::Stone);
        chunk.set(LocalBlockPos::new(5, 6, 7).unwrap(), BlockId::Dirt);
        let encoded = encode_chunk(&chunk, &index).unwrap();
        let decoded = decode_chunk(position, &encoded, &restored_index).unwrap();
        assert_eq!(decoded, chunk);
    }

    #[test]
    fn region_files_hold_multiple_chunks_with_negative_coordinates() {
        let mut chunks = BTreeMap::new();
        chunks.insert(ChunkPos::new(-1, 0, 2), vec![1, 2, 3]);
        chunks.insert(ChunkPos::new(-8, -9, 15), vec![4, 5]);
        let bytes = encode_region(&chunks).unwrap();
        assert_eq!(decode_region(&bytes).unwrap(), chunks);
        assert_eq!(
            ChunkRegionPos::from_chunk(ChunkPos::new(-1, -8, 8)),
            ChunkRegionPos { x: -1, y: -1, z: 1 }
        );
    }
}
