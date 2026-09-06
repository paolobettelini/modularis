use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_chunk_world_api::{ServerChunkWorld,ServerChunkWorldApi};
use voxel_frame_kinematic_api::*;
use voxel_frame_movement_lib::*;
use tokio::task::JoinHandle;
pub struct ServerVoxelFrameKinematicMod;
impl VoxelFrameKinematicApi for ServerVoxelFrameKinematicMod {}
impl ServerVoxelFrameKinematicMod{
 pub fn init(bevy:&mut BevyMod,_world:&mut impl ServerChunkWorldApi)->Self{
  bevy.app.init_resource::<VoxelFrameTrajectories>().add_message::<SetVoxelFrameMotion>()
   .configure_sets(Update,(VoxelFrameMotionSet::Collect,VoxelFrameMotionSet::Apply).chain().before(server_player_registry_api::ServerPlayerMovementSet::Receive))
   .add_systems(Update,(collect.in_set(VoxelFrameMotionSet::Collect),advance.in_set(VoxelFrameMotionSet::Apply)));Self
 }
 pub fn run(&self)->Option<Vec<JoinHandle<()>>>{None}
}
fn collect(time:Res<Time>,world:Res<ServerChunkWorld>,mut requests:MessageReader<SetVoxelFrameMotion>,mut tracks:ResMut<VoxelFrameTrajectories>){
 for request in requests.read(){
  if request.frame.is_root()||!request.movement.valid(){continue;}
  if !request.initial_elapsed_seconds.is_finite()||request.initial_elapsed_seconds<0.0{continue;}
  let Some(frame)=world.frames().get(&request.scope,request.frame)else{continue;};
  tracks.0.insert((request.scope.clone(),request.frame),FrameTrajectory{start:frame.transform,target:request.target,movement:request.movement.clone(),started_at_seconds:time.elapsed_secs_f64()-request.initial_elapsed_seconds});
 }
}
fn advance(time:Res<Time>,world:Res<ServerChunkWorld>,mut tracks:ResMut<VoxelFrameTrajectories>){
 tracks.0.retain(|(scope,id),trajectory|{
  if world.frames().get(scope,*id).is_none(){return false;}
  let (pose,_done)=trajectory.sample(time.elapsed_secs_f64());
  // Keep completed trajectories so late viewers receive an unambiguous final pose.
  world.frames().set_transform(scope,*id,pose).is_ok()
 });
}
