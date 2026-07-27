use bevy::prelude::*;
use bevy_mod::BevyMod;
use chunk_api::Chunk;
use chunk_storage_binary_format_lib::{
    ChunkRegionPos, GlobalBlockIndex, decode_chunk, decode_region, encode_chunk, encode_region,
};
use server_chunk_storage_api::{
    ChunkStorageError, ChunkStorageFlushReport, ServerChunkStorage, ServerChunkStorageApi,
    ServerChunkStorageBackend, StoredChunkKey,
};
use server_world_catalog_api::{ServerWorldCatalog, ServerWorldCatalogApi, WorldDirectory};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
    sync::Mutex,
};
use tokio::task::JoinHandle;
use voxel_math_api::ChunkPos;
use world_instance_api::WorldInstanceId;

pub struct ServerChunkStorageFsImpl;

impl ServerChunkStorageFsImpl {
    pub fn init<C: ServerWorldCatalogApi>(bevy: &mut BevyMod, _catalog_api: &mut C) -> Self {
        let catalog = bevy.app.world().resource::<ServerWorldCatalog>().clone();
        let backend = FilesystemChunkStorage::open(catalog.worlds())
            .unwrap_or_else(|error| panic!("failed to initialize chunk storage: {error}"));
        bevy.app.insert_resource(ServerChunkStorage::new(backend));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerChunkStorageApi for ServerChunkStorageFsImpl {}

struct FilesystemChunkStorage {
    state: Mutex<FilesystemStorageState>,
}

struct FilesystemStorageState {
    worlds: HashMap<WorldInstanceId, FilesystemWorldState>,
}

struct FilesystemWorldState {
    directory: WorldDirectory,
    index: GlobalBlockIndex,
    regions: HashMap<RegionCacheKey, RegionBuffer>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct RegionCacheKey {
    source: String,
    region: ChunkRegionPos,
}

struct RegionBuffer {
    chunks: BTreeMap<ChunkPos, Vec<u8>>,
    dirty_chunks: HashSet<ChunkPos>,
}

impl FilesystemChunkStorage {
    fn open(worlds: Vec<WorldDirectory>) -> Result<Self, ChunkStorageError> {
        let mut state = FilesystemStorageState {
            worlds: HashMap::new(),
        };
        for directory in worlds {
            let chunk_root = directory.root.join("data/chunk");
            fs::create_dir_all(chunk_root.join("regions")).map_err(io_error)?;
            let index_path = chunk_root.join("index.bin");
            let mut index = if index_path.exists() {
                GlobalBlockIndex::decode(&fs::read(&index_path).map_err(io_error)?)
                    .map_err(format_error)?
            } else {
                GlobalBlockIndex::current()
            };
            let changed = index.reconcile_current_blocks();
            if changed || !index_path.exists() {
                atomic_write(&index_path, &index.encode().map_err(format_error)?)?;
            }
            state.worlds.insert(
                directory.instance.clone(),
                FilesystemWorldState {
                    directory,
                    index,
                    regions: HashMap::new(),
                },
            );
        }
        Ok(Self {
            state: Mutex::new(state),
        })
    }
}

impl ServerChunkStorageBackend for FilesystemChunkStorage {
    fn load(&self, key: &StoredChunkKey) -> Result<Option<Chunk>, ChunkStorageError> {
        let mut state = self
            .state
            .lock()
            .expect("filesystem chunk storage lock poisoned");
        let Some(world) = state.worlds.get_mut(&key.instance) else {
            return Ok(None);
        };
        let payload = region_buffer(world, &key.source, key.position)?
            .chunks
            .get(&key.position)
            .cloned();
        let Some(payload) = payload else {
            return Ok(None);
        };
        decode_chunk(key.position, &payload, &world.index)
            .map(Some)
            .map_err(format_error)
    }

    fn queue_store(&self, key: &StoredChunkKey, chunk: &Chunk) -> Result<bool, ChunkStorageError> {
        if chunk.position() != key.position {
            return Err(ChunkStorageError(format!(
                "chunk payload position {:?} does not match storage key {:?}",
                chunk.position(),
                key.position
            )));
        }
        let mut state = self
            .state
            .lock()
            .expect("filesystem chunk storage lock poisoned");
        let Some(world) = state.worlds.get_mut(&key.instance) else {
            return Ok(false);
        };
        let payload = encode_chunk(chunk, &world.index).map_err(format_error)?;
        let region = region_buffer(world, &key.source, key.position)?;
        region.chunks.insert(key.position, payload);
        region.dirty_chunks.insert(key.position);
        Ok(true)
    }

    fn flush(&self) -> Result<ChunkStorageFlushReport, ChunkStorageError> {
        let mut state = self
            .state
            .lock()
            .expect("filesystem chunk storage lock poisoned");
        flush_state(&mut state)
    }

    fn pending_chunks(&self) -> usize {
        self.state
            .lock()
            .expect("filesystem chunk storage lock poisoned")
            .worlds
            .values()
            .flat_map(|world| world.regions.values())
            .map(|region| region.dirty_chunks.len())
            .sum()
    }
}

impl Drop for FilesystemChunkStorage {
    fn drop(&mut self) {
        let state = self
            .state
            .get_mut()
            .expect("filesystem chunk storage lock poisoned during drop");
        if let Err(error) = flush_state(state) {
            eprintln!("failed to flush chunk storage while dropping backend: {error}");
        }
    }
}

fn region_buffer<'a>(
    world: &'a mut FilesystemWorldState,
    source: &str,
    position: ChunkPos,
) -> Result<&'a mut RegionBuffer, ChunkStorageError> {
    let key = RegionCacheKey {
        source: source.to_string(),
        region: ChunkRegionPos::from_chunk(position),
    };
    if !world.regions.contains_key(&key) {
        let path = region_path(&world.directory.root, &key);
        let chunks = if path.exists() {
            let chunks =
                decode_region(&fs::read(&path).map_err(io_error)?).map_err(format_error)?;
            if let Some(invalid) = chunks
                .keys()
                .find(|position| ChunkRegionPos::from_chunk(**position) != key.region)
            {
                return Err(ChunkStorageError(format!(
                    "chunk {invalid:?} is stored in the wrong region file '{}'",
                    path.display()
                )));
            }
            chunks
        } else {
            BTreeMap::new()
        };
        world.regions.insert(
            key.clone(),
            RegionBuffer {
                chunks,
                dirty_chunks: HashSet::new(),
            },
        );
    }
    Ok(world
        .regions
        .get_mut(&key)
        .expect("region was inserted immediately above"))
}

fn flush_state(
    state: &mut FilesystemStorageState,
) -> Result<ChunkStorageFlushReport, ChunkStorageError> {
    let mut report = ChunkStorageFlushReport::default();
    for world in state.worlds.values_mut() {
        let root = world.directory.root.clone();
        for (key, region) in &mut world.regions {
            if region.dirty_chunks.is_empty() {
                continue;
            }
            let bytes = encode_region(&region.chunks).map_err(format_error)?;
            atomic_write(&region_path(&root, key), &bytes)?;
            report.regions_written += 1;
            report.chunks_written += region.dirty_chunks.len();
            region.dirty_chunks.clear();
        }
    }
    Ok(report)
}

fn region_path(world_root: &Path, key: &RegionCacheKey) -> PathBuf {
    world_root
        .join("data/chunk/regions")
        .join(hex_component(&key.source))
        .join(format!(
            "r.{}.{}.{}.bin",
            key.region.x, key.region.y, key.region.z
        ))
}

fn hex_component(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len() * 2);
    for byte in value.as_bytes() {
        write!(&mut encoded, "{byte:02x}").expect("writing to a string cannot fail");
    }
    encoded
}

fn atomic_write(path: &Path, bytes: &[u8]) -> Result<(), ChunkStorageError> {
    let parent = path.parent().ok_or_else(|| {
        ChunkStorageError(format!("storage path '{}' has no parent", path.display()))
    })?;
    fs::create_dir_all(parent).map_err(io_error)?;
    let temporary = path.with_extension("tmp");
    fs::write(&temporary, bytes).map_err(io_error)?;
    fs::rename(&temporary, path).map_err(io_error)
}

fn format_error(error: impl std::fmt::Display) -> ChunkStorageError {
    ChunkStorageError(error.to_string())
}

fn io_error(error: std::io::Error) -> ChunkStorageError {
    ChunkStorageError(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use block_instance_api::BlockId;
    use server_world_catalog_api::WorldId;
    use std::time::{SystemTime, UNIX_EPOCH};
    use voxel_math_api::LocalBlockPos;

    #[test]
    fn pending_chunks_are_readable_and_survive_a_flush_and_reopen() {
        let root = temporary_world_root();
        let instance = WorldInstanceId::new("test:persistent-world");
        let directory = WorldDirectory {
            id: WorldId::new("persistent-world").unwrap(),
            instance: instance.clone(),
            root: root.clone(),
        };
        let storage = FilesystemChunkStorage::open(vec![directory.clone()]).unwrap();
        let position = ChunkPos::new(-1, 4, 7);
        let key = StoredChunkKey {
            instance: instance.clone(),
            source: "test:terrain".to_string(),
            position,
        };
        let mut chunk = Chunk::filled(position, BlockId::Air);
        chunk.set(LocalBlockPos::new(3, 5, 7).unwrap(), BlockId::Obsidian);

        assert!(storage.queue_store(&key, &chunk).unwrap());
        assert_eq!(storage.pending_chunks(), 1);
        assert_eq!(storage.load(&key).unwrap(), Some(chunk.clone()));
        let report = storage.flush().unwrap();
        assert_eq!(report.regions_written, 1);
        assert_eq!(report.chunks_written, 1);
        drop(storage);

        let reopened = FilesystemChunkStorage::open(vec![directory]).unwrap();
        assert_eq!(reopened.load(&key).unwrap(), Some(chunk));
        assert!(root.join("data/chunk/index.bin").is_file());
        fs::remove_dir_all(root).unwrap();
    }

    fn temporary_world_root() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!(
            "patchwork-chunk-storage-{}-{unique}",
            std::process::id()
        ))
    }
}
