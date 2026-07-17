use bevy::prelude::Vec3;
use block_shape_api::BlockShape;
use voxel_math_api::BlockPos;

pub const PLAYER_HEIGHT: f32 = 1.8;
pub const PLAYER_RADIUS: f32 = 0.3;
pub const PLAYER_EYE_HEIGHT: f32 = 1.5;

pub fn player_intersects_block(player_position: [f32; 3], block: BlockPos) -> bool {
    player_intersects_shape(player_position, block, &BlockShape::full_cube())
}

pub fn player_intersects_shape(
    player_position: [f32; 3],
    block: BlockPos,
    shape: &BlockShape,
) -> bool {
    let player_min = [
        player_position[0] - PLAYER_RADIUS,
        player_position[1],
        player_position[2] - PLAYER_RADIUS,
    ];
    let player_max = [
        player_position[0] + PLAYER_RADIUS,
        player_position[1] + PLAYER_HEIGHT,
        player_position[2] + PLAYER_RADIUS,
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
}
