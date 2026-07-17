use bevy::prelude::*;
use voxel_math_api::BlockPos;

#[derive(Resource, Debug, Clone, Copy)]
pub struct ServerBlockInteractionRules {
    pub max_reach: f32,
    pub eye_height: f32,
}

impl ServerBlockInteractionRules {
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
