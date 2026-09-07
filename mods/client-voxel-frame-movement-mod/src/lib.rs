use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_voxel_frame_api::{ClientVoxelFrames,ClientVoxelFrameSet};
use client_voxel_frame_movement_api::{ClientFrameAnimations,ClientFrameAnimationSet};
use tokio::task::JoinHandle;
pub struct ClientVoxelFrameMovementMod;
impl ClientVoxelFrameMovementMod {
 pub fn init(bevy:&mut BevyMod)->Self {
  bevy.app.init_resource::<ClientVoxelFrames>().init_resource::<ClientFrameAnimations>()
   .add_systems(FixedUpdate,tick.in_set(ClientFrameAnimationSet).after(ClientVoxelFrameSet::Receive));Self
 }
 pub fn run(&self)->Option<Vec<JoinHandle<()>>>{None}
}
fn tick(time:Res<Time>,mut frames:ResMut<ClientVoxelFrames>,mut animations:ResMut<ClientFrameAnimations>){
 animations.0.retain(|id,animation|{
  let Some(scope)=&frames.scope else{return false;};
  let Some(mut frame)=frames.registry.get(scope,*id) else{return false;};
  if !animation.affects_collision{return true;}
  let (pose,done)=animation.movement.sample(animation.start,animation.target,time.elapsed_secs_f64()-animation.started_at);
  frame.transform=pose;frames.upsert(frame);
  // Keep the final interpolation interval available to the delayed renderer.
  !done || !animation.movement.sample(animation.start,animation.target,
      (time.elapsed_secs_f64()-animation.started_at-time.delta_secs_f64()).max(0.0)).1
 });
}
