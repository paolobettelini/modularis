use bevy::prelude::*;
use collision_api::Aabb;
use voxel_math_api::BlockPos;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VoxelRayHit {
    pub block: BlockPos,
    pub adjacent: BlockPos,
    pub normal: IVec3,
    pub distance: f32,
}

pub fn raycast_voxels(
    origin: Vec3,
    direction: Vec3,
    max_distance: f32,
    mut is_target: impl FnMut(BlockPos) -> bool,
) -> Option<VoxelRayHit> {
    let direction = direction.normalize_or_zero();
    if direction == Vec3::ZERO || max_distance <= 0.0 {
        return None;
    }

    let mut current = BlockPos::new(
        origin.x.floor() as i32,
        origin.y.floor() as i32,
        origin.z.floor() as i32,
    );
    let step = IVec3::new(
        direction.x.signum() as i32,
        direction.y.signum() as i32,
        direction.z.signum() as i32,
    );
    let delta = Vec3::new(
        axis_delta(direction.x),
        axis_delta(direction.y),
        axis_delta(direction.z),
    );
    let mut next = Vec3::new(
        axis_first_crossing(origin.x, direction.x, current.x),
        axis_first_crossing(origin.y, direction.y, current.y),
        axis_first_crossing(origin.z, direction.z, current.z),
    );

    if is_target(current) {
        return Some(VoxelRayHit {
            block: current,
            adjacent: current,
            normal: IVec3::ZERO,
            distance: 0.0,
        });
    }

    loop {
        let distance = next.x.min(next.y).min(next.z);
        if distance > max_distance {
            return None;
        }

        // Only advance multiple axes for a real grid-edge/grid-corner crossing.
        // A broad epsilon turns a ray that merely passes close to an edge into a
        // diagonal step and can report the wrong placement face.
        let crosses_x = next.x == distance;
        let crosses_y = next.y == distance;
        let crosses_z = next.z == distance;

        let primary_axis = primary_crossed_axis(direction, crosses_x, crosses_y, crosses_z);
        if crosses_x {
            next.x += delta.x;
            current.x += step.x;
        }
        if crosses_y {
            next.y += delta.y;
            current.y += step.y;
        }
        if crosses_z {
            next.z += delta.z;
            current.z += step.z;
        }

        if is_target(current) {
            let normal = match primary_axis {
                0 => IVec3::new(-step.x, 0, 0),
                1 => IVec3::new(0, -step.y, 0),
                _ => IVec3::new(0, 0, -step.z),
            };
            return Some(VoxelRayHit {
                block: current,
                adjacent: BlockPos::new(
                    current.x + normal.x,
                    current.y + normal.y,
                    current.z + normal.z,
                ),
                normal,
                distance,
            });
        }
    }
}

/// Traverses voxel cells with DDA, but intersects the exact union of local
/// AABBs returned for each cell instead of treating every target as a cube.
pub fn raycast_voxel_shapes<S: AsRef<[Aabb]>>(
    origin: Vec3,
    direction: Vec3,
    max_distance: f32,
    mut shape_at: impl FnMut(BlockPos) -> S,
) -> Option<VoxelRayHit> {
    let direction = direction.normalize_or_zero();
    if direction == Vec3::ZERO || max_distance <= 0.0 {
        return None;
    }

    let mut current = BlockPos::new(
        origin.x.floor() as i32,
        origin.y.floor() as i32,
        origin.z.floor() as i32,
    );
    let step = IVec3::new(
        direction.x.signum() as i32,
        direction.y.signum() as i32,
        direction.z.signum() as i32,
    );
    let delta = Vec3::new(
        axis_delta(direction.x),
        axis_delta(direction.y),
        axis_delta(direction.z),
    );
    let mut next = Vec3::new(
        axis_first_crossing(origin.x, direction.x, current.x),
        axis_first_crossing(origin.y, direction.y, current.y),
        axis_first_crossing(origin.z, direction.z, current.z),
    );
    let mut entered_at = 0.0;

    loop {
        let exits_at = next.x.min(next.y).min(next.z).min(max_distance);
        let shape = shape_at(current);
        if let Some((distance, normal)) = nearest_shape_hit(
            origin,
            direction,
            current,
            shape.as_ref(),
            entered_at,
            exits_at,
        ) {
            return Some(VoxelRayHit {
                block: current,
                adjacent: BlockPos::new(
                    current.x + normal.x,
                    current.y + normal.y,
                    current.z + normal.z,
                ),
                normal,
                distance,
            });
        }

        let distance = next.x.min(next.y).min(next.z);
        if distance > max_distance {
            return None;
        }
        let crosses_x = next.x == distance;
        let crosses_y = next.y == distance;
        let crosses_z = next.z == distance;
        if crosses_x {
            next.x += delta.x;
            current.x += step.x;
        }
        if crosses_y {
            next.y += delta.y;
            current.y += step.y;
        }
        if crosses_z {
            next.z += delta.z;
            current.z += step.z;
        }
        entered_at = distance;
    }
}

fn nearest_shape_hit(
    origin: Vec3,
    direction: Vec3,
    block: BlockPos,
    boxes: &[Aabb],
    entered_at: f32,
    exits_at: f32,
) -> Option<(f32, IVec3)> {
    const RAY_EPSILON: f32 = 1.0e-5;
    let offset = Vec3::new(block.x as f32, block.y as f32, block.z as f32);
    boxes
        .iter()
        .filter_map(|bounds| ray_aabb(origin, direction, bounds.min + offset, bounds.max + offset))
        .filter(|(distance, _)| {
            *distance + RAY_EPSILON >= entered_at && *distance <= exits_at + RAY_EPSILON
        })
        .min_by(|left, right| left.0.total_cmp(&right.0))
}

fn ray_aabb(origin: Vec3, direction: Vec3, min: Vec3, max: Vec3) -> Option<(f32, IVec3)> {
    let mut near = 0.0_f32;
    let mut far = f32::INFINITY;
    let mut normal = IVec3::ZERO;

    for axis in 0..3 {
        let origin_axis = origin[axis];
        let direction_axis = direction[axis];
        if direction_axis.abs() <= f32::EPSILON {
            if origin_axis < min[axis] || origin_axis > max[axis] {
                return None;
            }
            continue;
        }

        let (axis_near, axis_far, axis_normal) = if direction_axis > 0.0 {
            (
                (min[axis] - origin_axis) / direction_axis,
                (max[axis] - origin_axis) / direction_axis,
                negative_axis(axis),
            )
        } else {
            (
                (max[axis] - origin_axis) / direction_axis,
                (min[axis] - origin_axis) / direction_axis,
                positive_axis(axis),
            )
        };
        if axis_near > near {
            near = axis_near;
            normal = axis_normal;
        }
        far = far.min(axis_far);
        if near > far {
            return None;
        }
    }

    (far >= 0.0).then_some((near.max(0.0), normal))
}

fn negative_axis(axis: usize) -> IVec3 {
    match axis {
        0 => IVec3::NEG_X,
        1 => IVec3::NEG_Y,
        _ => IVec3::NEG_Z,
    }
}

fn positive_axis(axis: usize) -> IVec3 {
    match axis {
        0 => IVec3::X,
        1 => IVec3::Y,
        _ => IVec3::Z,
    }
}

fn primary_crossed_axis(
    direction: Vec3,
    crosses_x: bool,
    crosses_y: bool,
    crosses_z: bool,
) -> usize {
    let mut axis = 0;
    let mut magnitude = if crosses_x {
        direction.x.abs()
    } else {
        f32::NEG_INFINITY
    };
    if crosses_y && direction.y.abs() > magnitude {
        axis = 1;
        magnitude = direction.y.abs();
    }
    if crosses_z && direction.z.abs() > magnitude {
        axis = 2;
    }
    axis
}

fn axis_delta(direction: f32) -> f32 {
    if direction == 0.0 {
        f32::INFINITY
    } else {
        direction.recip().abs()
    }
}

fn axis_first_crossing(origin: f32, direction: f32, voxel: i32) -> f32 {
    if direction > 0.0 {
        (voxel as f32 + 1.0 - origin) / direction
    } else if direction < 0.0 {
        (origin - voxel as f32) / -direction
    } else {
        f32::INFINITY
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_hit_and_previous_empty_voxel() {
        let hit = raycast_voxels(Vec3::new(0.5, 0.5, 0.5), Vec3::X, 10.0, |position| {
            position == BlockPos::new(3, 0, 0)
        })
        .unwrap();
        assert_eq!(hit.block, BlockPos::new(3, 0, 0));
        assert_eq!(hit.adjacent, BlockPos::new(2, 0, 0));
        assert_eq!(hit.normal, IVec3::NEG_X);
    }

    #[test]
    fn does_not_visit_zero_width_voxels_on_an_exact_edge() {
        let hit = raycast_voxels(
            Vec3::new(0.5, 0.5, 0.5),
            Vec3::new(1.0, 1.0, 0.0),
            10.0,
            |position| position == BlockPos::new(1, 0, 0) || position == BlockPos::new(1, 1, 0),
        )
        .unwrap();

        assert_eq!(hit.block, BlockPos::new(1, 1, 0));
        assert_eq!(hit.adjacent, BlockPos::new(0, 1, 0));
        assert_eq!(hit.normal, IVec3::NEG_X);
    }

    #[test]
    fn a_tiny_offset_selects_the_actual_side_voxel() {
        let hit = raycast_voxels(
            Vec3::new(0.5, 0.5 - 1.0e-3, 0.5),
            Vec3::new(1.0, 1.0, 0.0),
            10.0,
            |position| position == BlockPos::new(1, 0, 0),
        )
        .unwrap();
        assert_eq!(hit.block, BlockPos::new(1, 0, 0));
    }

    #[test]
    fn a_subpixel_offset_is_not_treated_as_an_exact_edge() {
        let hit = raycast_voxels(
            Vec3::new(0.5, 0.5 - 1.0e-6, 0.5),
            Vec3::new(1.0, 1.0, 0.0),
            10.0,
            |position| position == BlockPos::new(1, 0, 0),
        )
        .unwrap();

        assert_eq!(hit.block, BlockPos::new(1, 0, 0));
        assert_eq!(hit.adjacent, BlockPos::new(0, 0, 0));
        assert_eq!(hit.normal, IVec3::NEG_X);
    }

    #[test]
    fn shape_raycast_passes_through_the_empty_half_of_a_slab() {
        let slab = [Aabb {
            min: Vec3::ZERO,
            max: Vec3::new(1.0, 0.5, 1.0),
        }];
        let hit = raycast_voxel_shapes(Vec3::new(-1.0, 0.75, 0.5), Vec3::X, 4.0, |position| {
            if position == BlockPos::new(0, 0, 0) {
                slab.to_vec()
            } else {
                Vec::new()
            }
        });

        assert_eq!(hit, None);
    }

    #[test]
    fn shape_raycast_hits_the_real_top_face() {
        let slab = [Aabb {
            min: Vec3::ZERO,
            max: Vec3::new(1.0, 0.5, 1.0),
        }];
        let hit = raycast_voxel_shapes(Vec3::new(0.5, 2.0, 0.5), Vec3::NEG_Y, 4.0, |position| {
            if position == BlockPos::new(0, 0, 0) {
                slab.to_vec()
            } else {
                Vec::new()
            }
        })
        .unwrap();

        assert_eq!(hit.block, BlockPos::new(0, 0, 0));
        assert_eq!(hit.adjacent, BlockPos::new(0, 1, 0));
        assert_eq!(hit.normal, IVec3::Y);
        assert!((hit.distance - 1.5).abs() < 1.0e-5);
    }
}
