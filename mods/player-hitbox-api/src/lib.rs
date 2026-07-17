use bevy::prelude::{Resource, Vec3};
use block_shape_api::BlockShape;
use voxel_math_api::BlockPos;

pub const PLAYER_HEIGHT: f32 = 1.8;
pub const PLAYER_RADIUS: f32 = 0.3;
pub const PLAYER_EYE_HEIGHT: f32 = 1.5;

#[derive(Resource, Debug, Clone, Copy, PartialEq)]
pub struct PlayerHitbox {
    pub radius: f32,
    pub height: f32,
    pub eye_height: f32,
}

impl Default for PlayerHitbox {
    fn default() -> Self {
        Self {
            radius: PLAYER_RADIUS,
            height: PLAYER_HEIGHT,
            eye_height: PLAYER_EYE_HEIGHT,
        }
    }
}

impl PlayerHitbox {
    pub fn scaled(self, scale: f32) -> Self {
        Self {
            radius: self.radius * scale,
            height: self.height * scale,
            eye_height: self.eye_height * scale,
        }
    }

    pub fn is_valid(self) -> bool {
        self.radius.is_finite()
            && self.height.is_finite()
            && self.eye_height.is_finite()
            && self.radius > 0.0
            && self.height > 0.0
            && self.eye_height >= 0.0
            && self.eye_height <= self.height
    }
}

pub trait PlayerHitboxApi: Send + Sync + 'static {}

pub fn player_intersects_block(player_position: [f32; 3], block: BlockPos) -> bool {
    player_intersects_shape_with_hitbox(
        player_position,
        PlayerHitbox::default(),
        block,
        &BlockShape::full_cube(),
    )
}

pub fn player_intersects_shape(
    player_position: [f32; 3],
    block: BlockPos,
    shape: &BlockShape,
) -> bool {
    player_intersects_shape_with_hitbox(player_position, PlayerHitbox::default(), block, shape)
}

pub fn player_intersects_shape_with_hitbox(
    player_position: [f32; 3],
    hitbox: PlayerHitbox,
    block: BlockPos,
    shape: &BlockShape,
) -> bool {
    let player_min = [
        player_position[0] - hitbox.radius,
        player_position[1],
        player_position[2] - hitbox.radius,
    ];
    let player_max = [
        player_position[0] + hitbox.radius,
        player_position[1] + hitbox.height,
        player_position[2] + hitbox.radius,
    ];
    let origin = Vec3::new(block.x as f32, block.y as f32, block.z as f32);
    shape.boxes().iter().any(|bounds| {
        let block_min = origin + bounds.min;
        let block_max = origin + bounds.max;
        overlaps(player_min[0], player_max[0], block_min.x, block_max.x)
            && overlaps(player_min[1], player_max[1], block_min.y, block_max.y)
            && overlaps(player_min[2], player_max[2], block_min.z, block_max.z)
    })
}

fn overlaps(a_min: f32, a_max: f32, b_min: f32, b_max: f32) -> bool {
    a_min < b_max && a_max > b_min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_blocks_inside_player_but_not_floor_contact() {
        let player = [0.5, 1.0, 0.5];
        assert!(player_intersects_block(player, BlockPos::new(0, 1, 0)));
        assert!(player_intersects_block(player, BlockPos::new(0, 2, 0)));
        assert!(!player_intersects_block(player, BlockPos::new(0, 0, 0)));
    }

    #[test]
    fn uses_each_box_in_a_partial_shape() {
        let lower_half = BlockShape::new([collision_api::Aabb {
            min: Vec3::ZERO,
            max: Vec3::new(1.0, 0.5, 1.0),
        }]);

        assert!(!player_intersects_shape(
            [0.5, 0.6, 0.5],
            BlockPos::new(0, 0, 0),
            &lower_half,
        ));
        assert!(player_intersects_shape(
            [0.5, 0.4, 0.5],
            BlockPos::new(0, 0, 0),
            &lower_half,
        ));
    }

    #[test]
    fn scaled_hitbox_fits_below_a_one_block_ceiling() {
        let player = [0.5, 0.0, 0.5];
        let ceiling = BlockPos::new(0, 1, 0);
        assert!(player_intersects_block(player, ceiling));
        assert!(!player_intersects_shape_with_hitbox(
            player,
            PlayerHitbox::default().scaled(0.5),
            ceiling,
            &BlockShape::full_cube(),
        ));
    }
}
