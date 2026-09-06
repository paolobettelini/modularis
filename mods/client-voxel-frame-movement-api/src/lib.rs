use bevy::prelude::*;
use std::collections::HashMap;
use voxel_frame_api::{VoxelFrameId,VoxelFrameTransform};
use voxel_frame_movement_lib::FrameMovement;
#[derive(Clone)]
pub struct FrameAnimation {pub affects_collision:bool,pub start:VoxelFrameTransform,pub target:VoxelFrameTransform,pub movement:FrameMovement,pub started_at:f64}
#[derive(Resource,Default)]
pub struct ClientFrameAnimations(pub HashMap<VoxelFrameId,FrameAnimation>);
#[derive(SystemSet,Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub struct ClientFrameAnimationSet;
