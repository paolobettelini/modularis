use bevy::prelude::*;
use voxel_frame_api::{VoxelFrameId,VoxelFrameTransform};
use voxel_frame_movement_lib::FrameMovement;
/// Presentation request; no authoritative simulation. The adapter supplies epoch and sequence.
#[derive(Message,Debug,Clone)]
pub struct SendVoxelFrameMotion {
 pub player_id:u64,
 pub frame:VoxelFrameId,
 pub target:VoxelFrameTransform,
 pub movement:FrameMovement,
}
