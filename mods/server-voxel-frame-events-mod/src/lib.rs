use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_chunk_world_api::{ServerChunkWorld,ServerChunkWorldApi};
use voxel_frame_registry_api::{VoxelFrameChanged,VoxelFrameSet};
use tokio::task::JoinHandle;
pub struct ServerVoxelFrameEventsMod;
impl ServerVoxelFrameEventsMod {
    pub fn init<W:ServerChunkWorldApi>(bevy:&mut BevyMod,_world:&mut W)->Self {
        bevy.app.add_message::<VoxelFrameChanged>().add_systems(PostUpdate,publish.in_set(VoxelFrameSet::CollectChanges));
        Self
    }
    pub fn run(&self)->Option<Vec<JoinHandle<()>>>{None}
}
fn publish(world:Res<ServerChunkWorld>,mut changes:MessageWriter<VoxelFrameChanged>) {
    for change in world.frames().drain_changes() { changes.write(change); }
}
