use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_shape_api::{BlockShapeApi,BlockShapeService};
use server_chunk_world_api::{ServerChunkWorldApi,ServerChunkWorld};
use server_block_placement_api::{PendingBlockPlacements,BlockPlacementSet};
use tokio::task::JoinHandle;
pub struct ServerVoxelOverlapValidationVanillaMod;
impl ServerVoxelOverlapValidationVanillaMod {
    pub fn init<W:ServerChunkWorldApi,S:BlockShapeApi>(bevy:&mut BevyMod,_world:&mut W,_shapes:&mut S)->Self {
        bevy.app.init_resource::<PendingBlockPlacements>().add_systems(Update,validate.in_set(BlockPlacementSet::Validate));Self
    }
    pub fn run(&self)->Option<Vec<JoinHandle<()>>>{None}
}
fn validate(world:Res<ServerChunkWorld>,shapes:Res<BlockShapeService>,mut pending:ResMut<PendingBlockPlacements>) {
    for index in 0..pending.0.len() {
        let (earlier,current)=pending.0.split_at_mut(index);
        let intent=&mut current[0];
        if intent.allowed && (server_voxel_overlap_lib::overlaps_other_frame(&world,&shapes,intent)
            || earlier.iter().any(|other|other.allowed && server_voxel_overlap_lib::placements_overlap(&world,&shapes,intent,other))) {
            intent.allowed=false;
        }
    }
}
