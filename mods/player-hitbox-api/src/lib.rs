use voxel_math_api::BlockPos;

pub const PLAYER_HEIGHT: f32 = 1.8;
pub const PLAYER_RADIUS: f32 = 0.3;
pub const PLAYER_EYE_HEIGHT: f32 = 1.5;

pub fn player_intersects_block(player_position: [f32; 3], block: BlockPos) -> bool {
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
    let block_min = [block.x as f32, block.y as f32, block.z as f32];
    let block_max = [
        block.x as f32 + 1.0,
        block.y as f32 + 1.0,
        block.z as f32 + 1.0,
    ];

    overlaps(player_min[0], player_max[0], block_min[0], block_max[0])
        && overlaps(player_min[1], player_max[1], block_min[1], block_max[1])
        && overlaps(player_min[2], player_max[2], block_min[2], block_max[2])
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
}
