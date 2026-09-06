use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_chunk_world_api::{ServerChunkWorld,ServerChunkWorldApi};
use server_world_catalog_api::{ServerWorldCatalog,ServerWorldCatalogApi};
use server_world_data_storage_api::{ServerWorldDataStorage,ServerWorldDataStorageApi,WorldDataKey};
use voxel_frame_api::VoxelFrameId;
use voxel_frame_registry_api::{VoxelFrameChanged,VoxelFrameSet};
use voxel_math_api::ChunkPos;
use world_instance_api::WorldInstanceId;
use tokio::task::JoinHandle;
use std::collections::HashSet;

pub struct ServerVoxelFramePersistenceMod;
fn key(instance:WorldInstanceId)->WorldDataKey {
    WorldDataKey{instance,domain:"modularis:voxel-frames".into(),source:"catalogue".into(),frame:VoxelFrameId::ROOT,partition:ChunkPos::new(0,0,0)}
}
impl ServerVoxelFramePersistenceMod {
    pub fn init<W:ServerChunkWorldApi,C:ServerWorldCatalogApi,S:ServerWorldDataStorageApi>(bevy:&mut BevyMod,_world:&mut W,_catalog:&mut C,_storage:&mut S,_events:&mut server_voxel_frame_events_mod::ServerVoxelFrameEventsMod)->Self {
        let catalog=bevy.app.world().resource::<ServerWorldCatalog>().clone();
        let storage=bevy.app.world().resource::<ServerWorldDataStorage>().clone();
        let frames=bevy.app.world().resource::<ServerChunkWorld>().frames();
        for directory in catalog.worlds() {
            if let Some(bytes)=storage.load(&key(directory.instance.clone())).expect("cannot load voxel frame catalogue") {
                for frame in voxel_frame_storage_lib::decode(&bytes).expect("invalid voxel frame catalogue") {
                    assert_eq!(frame.scope.instance,directory.instance,"frame belongs to another world");
                    frames.upsert(frame).expect("invalid persisted frame");
                }
            }
        }
        frames.drain_changes();
        bevy.app.add_systems(PostUpdate,persist_changes.in_set(VoxelFrameSet::Persist).after(VoxelFrameSet::CollectChanges));
        Self
    }
    pub fn run(&self)->Option<Vec<JoinHandle<()>>>{None}
}
fn persist_changes(mut changes:MessageReader<VoxelFrameChanged>,world:Res<ServerChunkWorld>,storage:Res<ServerWorldDataStorage>,mut dirty:Local<HashSet<WorldInstanceId>>) {
    for change in changes.read() {
        dirty.insert(match change { VoxelFrameChanged::Upsert(frame)=>frame.scope.instance.clone(),VoxelFrameChanged::Removed{scope,..}=>scope.instance.clone() });
    }
    if dirty.is_empty() { return; }
    let frames=world.frames().all();
    dirty.retain(|instance| {
        let records=frames.iter().filter(|f|&f.scope.instance==instance).cloned().collect::<Vec<_>>();
        match voxel_frame_storage_lib::encode(&records).and_then(|bytes|storage.queue_store(&key(instance.clone()),Some(&bytes)).map_err(|e|e.to_string())) {
            Ok(_) => false, Err(error)=>{error!("cannot queue voxel frame catalogue: {error}");true}
        }
    });
}
