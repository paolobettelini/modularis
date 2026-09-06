use bevy::{math::DQuat,prelude::EulerRot};
use voxel_frame_api::*;
use voxel_math_api::BlockPos;
use server_chunk_provider_api::ChunkViewer;
use server_chunk_world_api::ServerChunkWorld;
/// Generic construction mechanic. The caller selects the initial block and scope.
pub fn spawn_frame(world:&ServerChunkWorld,viewer:ChunkViewer,position:[f64;3],rotation_degrees:[f64;3],block:block_state_api::BlockState)->Result<VoxelFrameId,String>{
 if !position.iter().chain(rotation_degrees.iter()).all(|v|v.is_finite()) {return Err("Position and rotation must be finite".into());}
 if position.iter().any(|v|v.abs()>i32::MAX as f64-16.0){return Err("Position is outside the supported voxel range".into());}
 let chunk=BlockPos::new(position[0].floor() as i32,position[1].floor() as i32,position[2].floor() as i32).chunk();
 let scope=world.resident_key(viewer,chunk).ok_or("World route unavailable")?.scope();
 let [x,y,z]=rotation_degrees.map(f64::to_radians);
 let pose=VoxelFrameTransform::new(position,DQuat::from_euler(EulerRot::XYZ,x,y,z).to_array())?;
 let id=VoxelFrameId::new();
 world.frames().upsert(VoxelFrame::new(id,scope.clone(),pose))?;
 if let Err(error)=world.set_block_for(viewer,VoxelBlockAddress::new(id,BlockPos::new(0,0,0)),block){
  world.frames().remove(&scope,id);return Err(format!("Cannot initialize frame: {error:?}"));
 }
 Ok(id)
}
