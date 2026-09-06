pub use voxel_frame_movement_lib::{Easing,RepeatMode,FrameMovement};
use serde::{Serialize,Deserialize};
use voxel_frame_api::{VoxelFrame,VoxelFrameId};
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct VoxelFrameUpsert { pub frame: VoxelFrame, pub movement_epoch: u64, pub stream_sequence: u64 }
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct VoxelFrameRemove { pub id: VoxelFrameId, pub movement_epoch: u64, pub stream_sequence: u64 }

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct VoxelFramePose {
    pub id: VoxelFrameId,
    pub transform: voxel_frame_api::VoxelFrameTransform,
    pub revision: u64,
    pub movement: FrameMovement,
    pub affects_collision: bool,
    /// Authoritative trajectory, sampled at server_time_seconds (None for visual-only requests).
    pub trajectory: Option<voxel_frame_movement_lib::FrameTrajectory>,
    /// Server clock sample for an optional client interpolation adapter.
    pub server_time_seconds: f64,
    pub movement_epoch: u64,
    pub stream_sequence: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn wire_addresses_distinguish_equal_local_coordinates() {
        let local=voxel_math_api::ChunkPos::new(-1,2,-3);
        let a=voxel_frame_api::VoxelChunkAddress::new(VoxelFrameId::new(),local);
        let b=voxel_frame_api::VoxelChunkAddress::new(VoxelFrameId::new(),local);
        let bytes=serde_json::to_vec(&[a,b]).unwrap();
        let restored:Vec<voxel_frame_api::VoxelChunkAddress>=serde_json::from_slice(&bytes).unwrap();
        assert_eq!(restored,vec![a,b]);assert_ne!(restored[0],restored[1]);
    }
}
