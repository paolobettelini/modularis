use bevy::prelude::info;
use chunk_api::Chunk;
use chunk_storage_binary_format_lib::{
    GlobalBlockIndex, decode_chunk, decode_region, storage_source_component,
};
use std::{collections::BTreeMap, fs, path::{Path, PathBuf}};
use voxel_math_api::ChunkPos;

const HUB_CHUNK_SOURCE: &str = "patchwork:primary";
//const HUB_SPAWN: [f32; 3] = [0.5, 56.0, 0.5];
const HUB_SPAWN: [f32; 3] = [1817.5, 41.0, 1044.5];

/// Loads the preconverted Hub world from Modularis' native chunk format.
///
/// The path deliberately belongs to the monolithic TheCrown application
/// policy. The generic chunk provider only receives an in-memory template and
/// remains independent from Minecraft import and filesystem layout concerns.
pub fn load_hub_world() -> Result<([f32; 3], Vec<Chunk>), String> {
    let root = hub_world_path();
    let index_path = root.join("data/chunk/index.bin");
    let index = GlobalBlockIndex::decode(
        &fs::read(&index_path)
            .map_err(|error| format!("failed to read {}: {error}", index_path.display()))?,
    ).map_err(|error| format!("invalid Hub block index: {error}"))?;
    let region_root = root
        .join("data/chunk/regions")
        .join(storage_source_component(HUB_CHUNK_SOURCE));
    let mut encoded_chunks = BTreeMap::<ChunkPos, Vec<u8>>::new();
    for entry in fs::read_dir(&region_root)
        .map_err(|error| format!("failed to read {}: {error}", region_root.display()))?
    {
        let entry = entry.map_err(|error| error.to_string())?;
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("bin") {
            continue;
        }
        let region = decode_region(&fs::read(&path).map_err(|error| error.to_string())?)
            .map_err(|error| format!("invalid Hub region {}: {error}", path.display()))?;
        for (position, payload) in region {
            if encoded_chunks.insert(position, payload).is_some() {
                return Err(format!("duplicate Hub chunk {position:?}"));
            }
        }
    }
    let chunks = encoded_chunks
        .into_iter()
        .map(|(position, payload)| decode_chunk(position, &payload, &index))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("invalid Hub chunk: {error}"))?;
    info!(
        "loaded TheCrown Hub world '{}' into RAM ({} chunks)",
        root.display(),
        chunks.len(),
    );
    Ok((HUB_SPAWN, chunks))
}

fn hub_world_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../thecrown/data/hub")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converted_hub_world_decodes_from_native_storage() {
        let (spawn, chunks) = load_hub_world().unwrap();
        assert_eq!(spawn, HUB_SPAWN);
        assert!(!chunks.is_empty());
    }
}
