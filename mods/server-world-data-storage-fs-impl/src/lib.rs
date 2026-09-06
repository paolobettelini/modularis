use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_world_catalog_api::{ServerWorldCatalog, ServerWorldCatalogApi, WorldDirectory};
use server_world_data_storage_api::{
    ServerWorldDataStorage, ServerWorldDataStorageApi, ServerWorldDataStorageBackend,
    WorldDataFlushReport, WorldDataKey, WorldDataStorageError,
};
use std::{collections::HashMap, fmt::Write as _, fs, path::{Path, PathBuf}, sync::Mutex};
use tokio::task::JoinHandle;
use world_instance_api::WorldInstanceId;

pub struct ServerWorldDataStorageFsImpl;

impl ServerWorldDataStorageFsImpl {
    pub fn init<C: ServerWorldCatalogApi>(bevy: &mut BevyMod, _catalog: &mut C) -> Self {
        let worlds = bevy.app.world().resource::<ServerWorldCatalog>().worlds();
        bevy.app.insert_resource(ServerWorldDataStorage::new(FilesystemWorldDataStorage::new(worlds)));
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}
impl ServerWorldDataStorageApi for ServerWorldDataStorageFsImpl {}

struct FilesystemWorldDataStorage {
    roots: HashMap<WorldInstanceId, PathBuf>,
    pending: Mutex<HashMap<WorldDataKey, Option<Vec<u8>>>>,
}

impl FilesystemWorldDataStorage {
    fn new(worlds: Vec<WorldDirectory>) -> Self {
        let roots = worlds.into_iter().map(|world| (world.instance, world.root)).collect();
        Self { roots, pending: Mutex::new(HashMap::new()) }
    }

    fn path(&self, key: &WorldDataKey) -> Option<PathBuf> {
        let root=self.roots.get(&key.instance)?.join("data").join(hex(&key.domain)).join(hex(&key.source));
        let root=if key.frame.is_root() { root } else { root.join("frames").join(key.frame.to_string()) };
        Some(root.join(format!("c.{}.{}.{}.bin",key.partition.x,key.partition.y,key.partition.z)))
    }
}

impl ServerWorldDataStorageBackend for FilesystemWorldDataStorage {
    fn load(&self, key: &WorldDataKey) -> Result<Option<Vec<u8>>, WorldDataStorageError> {
        if let Some(pending) = self.pending.lock().unwrap().get(key).cloned() { return Ok(pending); }
        let Some(path) = self.path(key) else { return Ok(None) };
        if !path.exists() { return Ok(None); }
        fs::read(path).map(Some).map_err(io_error)
    }

    fn queue_store(&self, key: &WorldDataKey, payload: Option<&[u8]>) -> Result<bool, WorldDataStorageError> {
        if self.path(key).is_none() { return Ok(false); }
        self.pending.lock().unwrap().insert(key.clone(), payload.map(<[u8]>::to_vec));
        Ok(true)
    }

    fn flush(&self) -> Result<WorldDataFlushReport, WorldDataStorageError> {
        let pending = std::mem::take(&mut *self.pending.lock().unwrap());
        let mut report = WorldDataFlushReport::default();
        for (key, payload) in pending {
            let Some(path) = self.path(&key) else { continue };
            match payload {
                Some(payload) => { atomic_write(&path, &payload)?; report.records_written += 1; }
                None if path.exists() => { fs::remove_file(path).map_err(io_error)?; report.records_deleted += 1; }
                None => {}
            }
        }
        Ok(report)
    }

    fn pending_records(&self) -> usize { self.pending.lock().unwrap().len() }
}

impl Drop for FilesystemWorldDataStorage {
    fn drop(&mut self) { if let Err(error) = self.flush() { eprintln!("failed to flush world data storage: {error}"); } }
}

fn hex(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len() * 2);
    for byte in value.as_bytes() { write!(&mut encoded, "{byte:02x}").unwrap(); }
    encoded
}

fn atomic_write(path: &Path, bytes: &[u8]) -> Result<(), WorldDataStorageError> {
    fs::create_dir_all(path.parent().ok_or_else(|| WorldDataStorageError("world data path has no parent".into()))?).map_err(io_error)?;
    let temporary = path.with_extension("tmp");
    fs::write(&temporary, bytes).map_err(io_error)?;
    fs::rename(temporary, path).map_err(io_error)
}
fn io_error(error: std::io::Error) -> WorldDataStorageError { WorldDataStorageError(error.to_string()) }
