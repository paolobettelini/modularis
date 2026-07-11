use bevy::prelude::*;
use collision_api::CollisionResult;
use voxel_math_api::BlockPos;

const SKIN: f32 = 0.001;

#[derive(Debug, Clone, Copy)]
enum Axis {
    X,
    Y,
    Z,
}

pub fn collides_at(
    position: Vec3,
    radius: f32,
    height: f32,
    is_solid: &impl Fn(BlockPos) -> bool,
) -> bool {
    solid_blocks_overlapping(position, radius, height, is_solid)
        .next()
        .is_some()
}

pub fn resolve_player_collision(
    position: Vec3,
    movement: Vec3,
    radius: f32,
    height: f32,
    is_solid: &impl Fn(BlockPos) -> bool,
) -> CollisionResult {
    let mut result_position = depenetrate(position, radius, height, is_solid);
    // Resolve the height axis first. During a jump this lets the player clear
    // the top of a block before planar movement is tested against its side.
    // Resolving X first made diagonal landings stick and alternate between a
    // side correction and a vertical correction on consecutive frames.
    let (y, hit_y) = resolve_axis(
        result_position,
        movement.y,
        Axis::Y,
        radius,
        height,
        is_solid,
    );
    result_position = y;
    let (x, hit_x) = resolve_axis(
        result_position,
        movement.x,
        Axis::X,
        radius,
        height,
        is_solid,
    );
    result_position = x;
    let (z, hit_z) = resolve_axis(
        result_position,
        movement.z,
        Axis::Z,
        radius,
        height,
        is_solid,
    );

    CollisionResult {
        position: z,
        grounded: hit_y && movement.y < 0.0,
        hit_x,
        hit_y,
        hit_z,
    }
}

fn resolve_axis(
    position: Vec3,
    delta: f32,
    axis: Axis,
    radius: f32,
    height: f32,
    is_solid: &impl Fn(BlockPos) -> bool,
) -> (Vec3, bool) {
    if delta.abs() <= f32::EPSILON {
        return (position, false);
    }

    let mut candidate = position;
    set_axis(&mut candidate, axis, axis_value(position, axis) + delta);
    if !collides_at(candidate, radius, height, is_solid) {
        return (candidate, false);
    }

    let mut resolved_axis = axis_value(candidate, axis);
    for block in solid_blocks_overlapping(candidate, radius, height, is_solid) {
        if delta > 0.0 {
            resolved_axis = resolved_axis
                .min(block_min_axis(block, axis) - max_extent(axis, radius, height) - SKIN);
        } else {
            resolved_axis = resolved_axis
                .max(block_max_axis(block, axis) - min_extent(axis, radius, height) + SKIN);
        }
    }

    let start_axis = axis_value(position, axis);
    if delta > 0.0 {
        resolved_axis = resolved_axis.clamp(start_axis, axis_value(candidate, axis));
    } else {
        resolved_axis = resolved_axis.clamp(axis_value(candidate, axis), start_axis);
    }

    let mut resolved = position;
    set_axis(&mut resolved, axis, resolved_axis);
    if collides_at(resolved, radius, height, is_solid) {
        resolved = binary_search_axis(position, candidate, axis, radius, height, is_solid);
    }
    (resolved, true)
}

fn depenetrate(
    mut position: Vec3,
    radius: f32,
    height: f32,
    is_solid: &impl Fn(BlockPos) -> bool,
) -> Vec3 {
    for _ in 0..6 {
        let overlaps =
            solid_blocks_overlapping(position, radius, height, is_solid).collect::<Vec<_>>();
        if overlaps.is_empty() {
            return position;
        }

        let aabb_min = player_min(position, radius);
        let aabb_max = player_max(position, radius, height);
        let mut best: Option<Vec3> = None;
        for block in overlaps {
            let block_min = Vec3::new(block.x as f32, block.y as f32, block.z as f32);
            let block_max = block_min + Vec3::ONE;
            let candidates = [
                Vec3::new(block_max.x - aabb_min.x + SKIN, 0.0, 0.0),
                Vec3::new(block_min.x - aabb_max.x - SKIN, 0.0, 0.0),
                Vec3::new(0.0, block_max.y - aabb_min.y + SKIN, 0.0),
                Vec3::new(0.0, block_min.y - aabb_max.y - SKIN, 0.0),
                Vec3::new(0.0, 0.0, block_max.z - aabb_min.z + SKIN),
                Vec3::new(0.0, 0.0, block_min.z - aabb_max.z - SKIN),
            ];
            for candidate in candidates {
                if candidate.length_squared() <= f32::EPSILON {
                    continue;
                }
                if best.is_none_or(|best| candidate.length_squared() < best.length_squared()) {
                    best = Some(candidate);
                }
            }
        }

        let Some(push) = best else {
            return position;
        };
        position += push;
    }
    position
}

fn binary_search_axis(
    start: Vec3,
    end: Vec3,
    axis: Axis,
    radius: f32,
    height: f32,
    is_solid: &impl Fn(BlockPos) -> bool,
) -> Vec3 {
    let mut low = axis_value(start, axis);
    let mut high = axis_value(end, axis);
    for _ in 0..12 {
        let mid = (low + high) * 0.5;
        let mut candidate = start;
        set_axis(&mut candidate, axis, mid);
        if collides_at(candidate, radius, height, is_solid) {
            high = mid;
        } else {
            low = mid;
        }
    }
    let mut result = start;
    set_axis(&mut result, axis, low);
    result
}

fn solid_blocks_overlapping(
    position: Vec3,
    radius: f32,
    height: f32,
    is_solid: &impl Fn(BlockPos) -> bool,
) -> impl Iterator<Item = BlockPos> + '_ {
    let min = player_min(position, radius);
    let max = player_max(position, radius, height);
    let min_x = (min.x + SKIN).floor() as i32;
    let min_y = (min.y + SKIN).floor() as i32;
    let min_z = (min.z + SKIN).floor() as i32;
    let max_x = (max.x - SKIN).floor() as i32;
    let max_y = (max.y - SKIN).floor() as i32;
    let max_z = (max.z - SKIN).floor() as i32;

    (min_y..=max_y).flat_map(move |y| {
        (min_z..=max_z).flat_map(move |z| {
            (min_x..=max_x).filter_map(move |x| {
                let position = BlockPos::new(x, y, z);
                is_solid(position).then_some(position)
            })
        })
    })
}

fn player_min(position: Vec3, radius: f32) -> Vec3 {
    Vec3::new(position.x - radius, position.y, position.z - radius)
}

fn player_max(position: Vec3, radius: f32, height: f32) -> Vec3 {
    Vec3::new(
        position.x + radius,
        position.y + height,
        position.z + radius,
    )
}

fn min_extent(axis: Axis, radius: f32, _height: f32) -> f32 {
    match axis {
        Axis::X | Axis::Z => -radius,
        Axis::Y => 0.0,
    }
}

fn max_extent(axis: Axis, radius: f32, height: f32) -> f32 {
    match axis {
        Axis::X | Axis::Z => radius,
        Axis::Y => height,
    }
}

fn axis_value(value: Vec3, axis: Axis) -> f32 {
    match axis {
        Axis::X => value.x,
        Axis::Y => value.y,
        Axis::Z => value.z,
    }
}

fn set_axis(value: &mut Vec3, axis: Axis, axis_value: f32) {
    match axis {
        Axis::X => value.x = axis_value,
        Axis::Y => value.y = axis_value,
        Axis::Z => value.z = axis_value,
    }
}

fn block_min_axis(block: BlockPos, axis: Axis) -> f32 {
    match axis {
        Axis::X => block.x as f32,
        Axis::Y => block.y as f32,
        Axis::Z => block.z as f32,
    }
}

fn block_max_axis(block: BlockPos, axis: Axis) -> f32 {
    block_min_axis(block, axis) + 1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertical_motion_clears_a_ledge_before_planar_motion() {
        let result = resolve_player_collision(
            Vec3::new(0.69, 1.001, 0.5),
            Vec3::new(0.4, 1.1, 0.0),
            0.3,
            1.8,
            &|position| position == BlockPos::new(1, 1, 0),
        );

        assert!(!result.hit_x);
        assert!(!result.hit_y);
        assert!(result.position.x > 1.0);
        assert!(result.position.y > 2.0);
    }

    #[test]
    fn landing_is_stable_on_the_following_frame() {
        let solid = |position| position == BlockPos::new(0, 0, 0);
        let landed = resolve_player_collision(
            Vec3::new(0.5, 1.2, 0.5),
            Vec3::new(0.0, -0.5, 0.0),
            0.3,
            1.8,
            &solid,
        );
        let stable = resolve_player_collision(landed.position, Vec3::ZERO, 0.3, 1.8, &solid);

        assert!(landed.grounded);
        assert!((landed.position.y - 1.001).abs() < 1.0e-4);
        assert_eq!(stable.position, landed.position);
        assert!(!collides_at(stable.position, 0.3, 1.8, &solid));
    }
}
