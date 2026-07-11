use bevy::prelude::*;
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
}
