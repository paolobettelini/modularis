use bevy_mod::BevyMod;
use server_world_data_storage_api::{ServerWorldDataStorage, ServerWorldDataStorageApi};
use tokio::task::JoinHandle;

pub struct ServerWorldDataStorageMemoryImpl;
impl ServerWorldDataStorageMemoryImpl {
    pub fn init(bevy: &mut BevyMod) -> Self { bevy.app.insert_resource(ServerWorldDataStorage::memory()); Self }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}
impl ServerWorldDataStorageApi for ServerWorldDataStorageMemoryImpl {}

