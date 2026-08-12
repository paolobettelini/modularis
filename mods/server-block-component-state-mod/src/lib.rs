use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_component_api::BlockComponentRegistry;
use block_component_registry_mod::BlockComponentRegistryMod;
use server_block_component_api::{ServerBlockComponentApi, ServerBlockComponentSet, ServerBlockComponents};
use server_block_component_persistence_lib::{load_if_needed, queue_dirty};
use server_chunk_world_api::{ServerChunkWorld, ServerChunkWorldApi};
use server_world_data_storage_api::{ServerWorldDataStorage, ServerWorldDataStorageApi};
use std::collections::HashSet;
use tokio::task::JoinHandle;

pub struct ServerBlockComponentStateMod;
impl ServerBlockComponentStateMod {
    pub fn init<W: ServerChunkWorldApi, S: ServerWorldDataStorageApi>(
        bevy: &mut BevyMod,
        _registry: &mut BlockComponentRegistryMod,
        _world: &mut W,
        _storage: &mut S,
    ) -> Self {
        bevy.app.init_resource::<ServerBlockComponents>()
            .configure_sets(Update, (ServerBlockComponentSet::Load, ServerBlockComponentSet::Mutate, ServerBlockComponentSet::Persist).chain())
            .add_systems(Update, load_resident.in_set(ServerBlockComponentSet::Load))
            .add_systems(Update, persist_dirty.in_set(ServerBlockComponentSet::Persist));
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}
impl ServerBlockComponentApi for ServerBlockComponentStateMod {}

fn load_resident(
    world: Res<ServerChunkWorld>,
    registry: Res<BlockComponentRegistry>,
    storage: Res<ServerWorldDataStorage>,
    mut components: ResMut<ServerBlockComponents>,
) {
    for key in world.resident_keys() {
        if let Err(error) = load_if_needed(&mut components, &key, &registry, &storage) {
            error!("failed to load block components for {:?}: {error}", key.position);
        }
    }
}

fn persist_dirty(
    world: Res<ServerChunkWorld>,
    registry: Res<BlockComponentRegistry>,
    storage: Res<ServerWorldDataStorage>,
    mut components: ResMut<ServerBlockComponents>,
) {
    if let Err(error) = queue_dirty(&mut components, &registry, &storage) {
        error!("failed to queue block component persistence: {error}");
    }
    let resident = world.resident_keys().into_iter().collect::<HashSet<_>>();
    components.unload_clean_except(&resident);
}
