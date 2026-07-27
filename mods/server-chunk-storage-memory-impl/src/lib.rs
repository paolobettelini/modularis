use bevy_mod::BevyMod;
use server_chunk_storage_api::{ServerChunkStorage, ServerChunkStorageApi};
use tokio::task::JoinHandle;

pub struct ServerChunkStorageMemoryImpl;

impl ServerChunkStorageMemoryImpl {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app.insert_resource(ServerChunkStorage::memory());
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerChunkStorageApi for ServerChunkStorageMemoryImpl {}
