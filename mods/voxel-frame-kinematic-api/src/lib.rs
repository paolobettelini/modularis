use bevy::prelude::*;
use std::collections::HashMap;
use voxel_frame_api::*;
use voxel_frame_movement_lib::*;
use world_instance_api::WorldScopeId;
#[derive(Message,Debug,Clone)]
pub struct SetVoxelFrameMotion {
 pub scope:WorldScopeId,
 pub frame:VoxelFrameId,
 pub target:VoxelFrameTransform,
 pub movement:FrameMovement,
 /// Starts a deterministic repeating trajectory at an existing phase. This
 /// offset is simulation state and is replicated through FrameTrajectory.
 pub initial_elapsed_seconds:f64,
}
#[derive(Resource,Default)]
pub struct VoxelFrameTrajectories(pub HashMap<(WorldScopeId,VoxelFrameId),FrameTrajectory>);
#[derive(SystemSet,Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub enum VoxelFrameMotionSet {Collect,Apply}
pub trait VoxelFrameKinematicApi: Send + Sync + 'static {}
