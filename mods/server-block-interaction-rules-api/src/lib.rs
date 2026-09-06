use bevy::prelude::*;
use voxel_math_api::BlockPos;

#[derive(Resource, Debug, Clone, Copy)]
pub struct ServerBlockInteractionRules {
    pub max_reach: f32,
    pub eye_height: f32,
}

impl ServerBlockInteractionRules {
    pub fn player_can_reach_in_frame(self, player_position: [f32;3], up: Vec3, eye_height: f32, block: voxel_frame_api::VoxelBlockAddress, pose: voxel_frame_api::VoxelFrameTransform) -> bool {
        let eye = (Vec3::from_array(player_position)+up.normalize_or_zero()*eye_height).as_dvec3();
        let local_eye = pose.world_to_local(eye);
        let min = bevy::math::DVec3::new(block.local.x as f64,block.local.y as f64,block.local.z as f64);
        let nearest = local_eye.clamp(min,min+bevy::math::DVec3::ONE);
        local_eye.distance_squared(nearest) <= (self.max_reach as f64).powi(2)
    }

    pub fn player_can_reach(self, player_position: [f32; 3], up: Vec3, block: BlockPos) -> bool {
        self.player_can_reach_from_eye(player_position, up, self.eye_height, block)
    }

    pub fn player_can_reach_from_eye(
        self,
        player_position: [f32; 3],
        up: Vec3,
        eye_height: f32,
        block: BlockPos,
    ) -> bool {
        let eye = Vec3::from_array(player_position) + up.normalize_or_zero() * eye_height;
        let center = Vec3::new(
            block.x as f32 + 0.5,
            block.y as f32 + 0.5,
            block.z as f32 + 0.5,
        );
        eye.distance_squared(center) <= self.max_reach * self.max_reach
    }
}

pub trait ServerBlockInteractionRulesApi: Send + Sync + 'static {}
