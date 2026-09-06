use serde::Serialize;
use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
    fs,
    path::Path,
};

const OUTPUT_SOURCE: &str = "patchwork:primary";
const FORMAT_VERSION: u16 = 2;
const CHUNK_VOLUME: usize = 16 * 16 * 16;
const EMPTY_BLOCK_STATE_CBOR: &[u8] = &[0xa0];
const REGION_EDGE_CHUNKS: i32 = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ChunkPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct RegionPos {
    x: i32,
    y: i32,
    z: i32,
}

impl RegionPos {
    fn from_chunk(chunk: ChunkPos) -> Self {
        Self {
            x: chunk.x.div_euclid(REGION_EDGE_CHUNKS),
            y: chunk.y.div_euclid(REGION_EDGE_CHUNKS),
            z: chunk.z.div_euclid(REGION_EDGE_CHUNKS),
        }
    }
}

pub struct NativeChunk {
    blocks: Vec<u32>,
}

impl NativeChunk {
    pub fn empty(air: u32) -> Self {
        Self {
            blocks: vec![air; CHUNK_VOLUME],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, block: u32) {
        self.blocks[x + z * 16 + y * 16 * 16] = block;
    }
}

#[derive(Debug, Serialize)]
struct WorldInfo {
    seed: u64,
}

pub fn write_world(
    output: &Path,
    seed: u64,
    block_ids: &[String],
    chunks: BTreeMap<ChunkPos, NativeChunk>,
) -> Result<(), Box<dyn Error>> {
    let chunk_root = output.join("data/chunk");
    let region_root = chunk_root
        .join("regions")
        .join(storage_source_component(OUTPUT_SOURCE));

    fs::create_dir_all(&region_root)?;
    fs::write(
        output.join("info.json"),
        serde_json::to_vec_pretty(&WorldInfo { seed })?,
    )?;
    fs::write(
        chunk_root.join("index.bin"),
        encode_global_index(block_ids)?,
    )?;

    let mut regions = BTreeMap::<RegionPos, BTreeMap<ChunkPos, Vec<u8>>>::new();
    for (position, chunk) in chunks {
        regions
            .entry(RegionPos::from_chunk(position))
            .or_default()
            .insert(position, encode_chunk(&chunk)?);
    }

    let mut chunk_count = 0usize;
    for (position, chunks) in regions {
        chunk_count += chunks.len();
        fs::write(
            region_root.join(format!(
                "r.{}.{}.{}.bin",
                position.x, position.y, position.z
            )),
            encode_region(&chunks)?,
        )?;
    }

    println!("Wrote {chunk_count} non-empty chunks");
    Ok(())
}

fn encode_global_index(ids: &[String]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"PWBI");
    write_u16(&mut bytes, FORMAT_VERSION);
    write_u32(&mut bytes, ids.len().try_into()?);

    for id in ids {
        write_u16(&mut bytes, id.len().try_into()?);
        bytes.extend_from_slice(id.as_bytes());
    }

    Ok(bytes)
}

fn encode_chunk(chunk: &NativeChunk) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut palette = Vec::<u32>::new();
    let mut reverse = HashMap::<u32, u32>::new();
    let mut entries = Vec::with_capacity(CHUNK_VOLUME);

    for block in &chunk.blocks {
        let next = palette.len() as u32;
        let local = *reverse.entry(*block).or_insert_with(|| {
            palette.push(*block);
            next
        });
        entries.push(local);
    }

    let bits = bits_required(palette.len());
    let words = pack_entries(&entries, bits);
    let mut bytes = Vec::new();

    write_u16(&mut bytes, palette.len().try_into()?);
    for global_index in palette {
        write_u32(&mut bytes, global_index);
        write_u32(&mut bytes, EMPTY_BLOCK_STATE_CBOR.len().try_into()?);
        bytes.extend_from_slice(EMPTY_BLOCK_STATE_CBOR);
    }

    bytes.push(bits);
    write_u32(&mut bytes, words.len().try_into()?);
    for word in words {
        bytes.extend_from_slice(&word.to_le_bytes());
    }

    Ok(bytes)
}

fn encode_region(chunks: &BTreeMap<ChunkPos, Vec<u8>>) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"PWCR");
    write_u16(&mut bytes, FORMAT_VERSION);
    write_u32(&mut bytes, chunks.len().try_into()?);

    for (position, payload) in chunks {
        for coordinate in [position.x, position.y, position.z] {
            bytes.extend_from_slice(&coordinate.to_le_bytes());
        }
        write_u32(&mut bytes, payload.len().try_into()?);
        bytes.extend_from_slice(payload);
    }

    Ok(bytes)
}

fn bits_required(value_count: usize) -> u8 {
    if value_count <= 1 {
        0
    } else {
        (usize::BITS - (value_count - 1).leading_zeros()) as u8
    }
}

fn pack_entries(entries: &[u32], bits: u8) -> Vec<u64> {
    if bits == 0 {
        return Vec::new();
    }

    let mut words = vec![0_u64; (entries.len() * bits as usize).div_ceil(64)];
    let mask = (1_u64 << bits) - 1;

    for (index, entry) in entries.iter().copied().enumerate() {
        let bit_index = index * bits as usize;
        let word_index = bit_index / 64;
        let bit_offset = bit_index % 64;
        let value = u64::from(entry) & mask;

        words[word_index] |= value << bit_offset;
        if bit_offset + bits as usize > 64 {
            words[word_index + 1] |= value >> (64 - bit_offset);
        }
    }

    words
}

fn storage_source_component(source: &str) -> String {
    source
        .as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn write_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn write_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}
