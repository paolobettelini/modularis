use bevy::prelude::*;
use block_shape_api::BlockShape;
use collision_api::{Aabb, CollisionResult};
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
    shape_at: &impl Fn(BlockPos) -> BlockShape,
) -> bool {
    !collision_boxes_overlapping(position, radius, height, shape_at).is_empty()
}

pub fn resolve_player_collision(
    position: Vec3,
    movement: Vec3,
    radius: f32,
    height: f32,
    shape_at: &impl Fn(BlockPos) -> BlockShape,
) -> CollisionResult {
    let mut result_position = depenetrate(position, radius, height, shape_at);
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
        shape_at,
    );
    result_position = y;
    let (x, hit_x) = resolve_axis(
        result_position,
        movement.x,
        Axis::X,
        radius,
        height,
        shape_at,
    );
    result_position = x;
    let (z, hit_z) = resolve_axis(
        result_position,
        movement.z,
        Axis::Z,
        radius,
        height,
        shape_at,
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
    shape_at: &impl Fn(BlockPos) -> BlockShape,
) -> (Vec3, bool) {
    if delta.abs() <= f32::EPSILON {
        return (position, false);
    }

    let mut candidate = position;
    set_axis(&mut candidate, axis, axis_value(position, axis) + delta);
    if !collides_at(candidate, radius, height, shape_at) {
        return (candidate, false);
    }

    let mut resolved_axis = axis_value(candidate, axis);
    for bounds in collision_boxes_overlapping(candidate, radius, height, shape_at) {
        if delta > 0.0 {
            resolved_axis = resolved_axis
                .min(aabb_min_axis(bounds, axis) - max_extent(axis, radius, height) - SKIN);
        } else {
            resolved_axis = resolved_axis
                .max(aabb_max_axis(bounds, axis) - min_extent(axis, radius, height) + SKIN);
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
    if collides_at(resolved, radius, height, shape_at) {
        resolved = binary_search_axis(position, candidate, axis, radius, height, shape_at);
    }
    (resolved, true)
}

fn depenetrate(
    mut position: Vec3,
    radius: f32,
    height: f32,
    shape_at: &impl Fn(BlockPos) -> BlockShape,
) -> Vec3 {
    for _ in 0..6 {
        let overlaps = collision_boxes_overlapping(position, radius, height, shape_at);
        if overlaps.is_empty() {
            return position;
        }

        let aabb_min = player_min(position, radius);
        let aabb_max = player_max(position, radius, height);
        let mut best: Option<Vec3> = None;
        for bounds in overlaps {
            let candidates = [
                Vec3::new(bounds.max.x - aabb_min.x + SKIN, 0.0, 0.0),
                Vec3::new(bounds.min.x - aabb_max.x - SKIN, 0.0, 0.0),
                Vec3::new(0.0, bounds.max.y - aabb_min.y + SKIN, 0.0),
                Vec3::new(0.0, bounds.min.y - aabb_max.y - SKIN, 0.0),
                Vec3::new(0.0, 0.0, bounds.max.z - aabb_min.z + SKIN),
                Vec3::new(0.0, 0.0, bounds.min.z - aabb_max.z - SKIN),
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
    shape_at: &impl Fn(BlockPos) -> BlockShape,
) -> Vec3 {
    let mut low = axis_value(start, axis);
    let mut high = axis_value(end, axis);
    for _ in 0..12 {
        let mid = (low + high) * 0.5;
        let mut candidate = start;
        set_axis(&mut candidate, axis, mid);
        if collides_at(candidate, radius, height, shape_at) {
            high = mid;
        } else {
            low = mid;
        }
    }
    let mut result = start;
    set_axis(&mut result, axis, low);
    result
}

fn collision_boxes_overlapping(
    position: Vec3,
    radius: f32,
    height: f32,
    shape_at: &impl Fn(BlockPos) -> BlockShape,
) -> Vec<Aabb> {
    let min = player_min(position, radius);
    let max = player_max(position, radius, height);
    // Model elements normally stay inside their cell. The one-cell margin also
    // supports authored elements that extend slightly beyond the 0..1 range.
    let min_x = (min.x + SKIN).floor() as i32 - 1;
    let min_y = (min.y + SKIN).floor() as i32 - 1;
    let min_z = (min.z + SKIN).floor() as i32 - 1;
    let max_x = (max.x - SKIN).floor() as i32 + 1;
    let max_y = (max.y - SKIN).floor() as i32 + 1;
    let max_z = (max.z - SKIN).floor() as i32 + 1;
    let mut output = Vec::new();

    for y in min_y..=max_y {
        for z in min_z..=max_z {
            for x in min_x..=max_x {
                let block = BlockPos::new(x, y, z);
                let origin = Vec3::new(x as f32, y as f32, z as f32);
                let shape = shape_at(block);
                for local in shape.boxes() {
                    let world = Aabb {
                        min: origin + local.min,
                        max: origin + local.max,
                    };
                    if aabb_overlaps(min, max, world.min, world.max) {
                        output.push(world);
                    }
                }
            }
        }
    }
    output
}

fn aabb_overlaps(a_min: Vec3, a_max: Vec3, b_min: Vec3, b_max: Vec3) -> bool {
    a_min.x < b_max.x - SKIN
        && a_max.x > b_min.x + SKIN
        && a_min.y < b_max.y - SKIN
        && a_max.y > b_min.y + SKIN
        && a_min.z < b_max.z - SKIN
        && a_max.z > b_min.z + SKIN
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

fn aabb_min_axis(bounds: Aabb, axis: Axis) -> f32 {
    match axis {
        Axis::X => bounds.min.x,
        Axis::Y => bounds.min.y,
        Axis::Z => bounds.min.z,
    }
}

fn aabb_max_axis(bounds: Aabb, axis: Axis) -> f32 {
    match axis {
        Axis::X => bounds.max.x,
        Axis::Y => bounds.max.y,
        Axis::Z => bounds.max.z,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shape_at(target: BlockPos, shape: BlockShape) -> impl Fn(BlockPos) -> BlockShape {
        move |position| {
            if position == target {
                shape.clone()
            } else {
                BlockShape::empty()
            }
        }
    }

    #[test]
    fn vertical_motion_clears_a_ledge_before_planar_motion() {
        let result = resolve_player_collision(
            Vec3::new(0.69, 1.001, 0.5),
            Vec3::new(0.4, 1.1, 0.0),
            0.3,
            1.8,
            &shape_at(BlockPos::new(1, 1, 0), BlockShape::full_cube()),
        );

        assert!(!result.hit_x);
        assert!(!result.hit_y);
        assert!(result.position.x > 1.0);
        assert!(result.position.y > 2.0);
    }

    #[test]
    fn landing_is_stable_on_the_following_frame() {
        let solid = shape_at(BlockPos::new(0, 0, 0), BlockShape::full_cube());
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

    #[test]
    fn lands_on_the_actual_top_of_a_partial_block_shape() {
        let lower_slab = BlockShape::new([Aabb {
            min: Vec3::ZERO,
            max: Vec3::new(1.0, 0.5, 1.0),
        }]);
        let shape = shape_at(BlockPos::new(0, 0, 0), lower_slab);
        let landed = resolve_player_collision(
            Vec3::new(0.5, 1.2, 0.5),
            Vec3::new(0.0, -1.0, 0.0),
            0.3,
            1.8,
            &shape,
        );

        assert!(landed.grounded);
        assert!((landed.position.y - 0.501).abs() < 1.0e-4);
    }
}
