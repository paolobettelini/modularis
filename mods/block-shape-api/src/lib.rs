use bevy::prelude::*;
use block_instance_api::BlockInstance;
use collision_api::Aabb;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlockShapeEdge {
    pub start: Vec3,
    pub end: Vec3,
    /// Component-wise direction in which an outline can be moved away from
    /// the solid. Concave edges point into the empty corner.
    pub expansion_direction: Vec3,
}

#[derive(Debug, Clone, PartialEq)]
struct BlockShapeGeometry {
    boxes: Arc<[Aabb]>,
    boundary_edges: Arc<[BlockShapeEdge]>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockShape(Arc<BlockShapeGeometry>);

impl BlockShape {
    pub fn new(boxes: impl Into<Arc<[Aabb]>>) -> Self {
        let boxes = boxes.into();
        let boundary_edges = external_boundary_edges(&boxes).into();
        Self(Arc::new(BlockShapeGeometry {
            boxes,
            boundary_edges,
        }))
    }

    pub fn empty() -> Self {
        Self::new(Arc::<[Aabb]>::from([]))
    }

    pub fn full_cube() -> Self {
        Self::new([Aabb {
            min: Vec3::ZERO,
            max: Vec3::ONE,
        }])
    }

    pub fn boxes(&self) -> &[Aabb] {
        &self.0.boxes
    }

    pub fn boundary_edges(&self) -> &[BlockShapeEdge] {
        &self.0.boundary_edges
    }

    pub fn is_empty(&self) -> bool {
        self.0.boxes.is_empty()
    }
}

impl AsRef<[Aabb]> for BlockShape {
    fn as_ref(&self) -> &[Aabb] {
        self.boxes()
    }
}

type BlockShapeFn = dyn Fn(&BlockInstance) -> BlockShape + Send + Sync + 'static;

#[derive(Resource, Clone)]
pub struct BlockShapeService {
    shape: Arc<BlockShapeFn>,
}

impl BlockShapeService {
    pub fn new(shape: impl Fn(&BlockInstance) -> BlockShape + Send + Sync + 'static) -> Self {
        Self {
            shape: Arc::new(shape),
        }
    }

    pub fn shape(&self, block: &BlockInstance) -> BlockShape {
        (self.shape)(block)
    }
}

pub trait BlockShapeApi: Send + Sync + 'static {}

fn external_boundary_edges(boxes: &[Aabb]) -> Vec<BlockShapeEdge> {
    if boxes.is_empty() {
        return Vec::new();
    }
    let xs = axis_coordinates(boxes, 0);
    let ys = axis_coordinates(boxes, 1);
    let zs = axis_coordinates(boxes, 2);
    if xs.len() < 2 || ys.len() < 2 || zs.len() < 2 {
        return Vec::new();
    }

    let nx = xs.len() - 1;
    let ny = ys.len() - 1;
    let nz = zs.len() - 1;
    let mut occupied = vec![false; nx * ny * nz];
    for bounds in boxes {
        let Some(x0) = coordinate_index(&xs, bounds.min.x) else {
            continue;
        };
        let Some(x1) = coordinate_index(&xs, bounds.max.x) else {
            continue;
        };
        let Some(y0) = coordinate_index(&ys, bounds.min.y) else {
            continue;
        };
        let Some(y1) = coordinate_index(&ys, bounds.max.y) else {
            continue;
        };
        let Some(z0) = coordinate_index(&zs, bounds.min.z) else {
            continue;
        };
        let Some(z1) = coordinate_index(&zs, bounds.max.z) else {
            continue;
        };
        for x in x0.min(x1)..x0.max(x1) {
            for y in y0.min(y1)..y0.max(y1) {
                for z in z0.min(z1)..z0.max(z1) {
                    occupied[(x * ny + y) * nz + z] = true;
                }
            }
        }
    }

    let cell = |x: isize, y: isize, z: isize| {
        if x < 0 || y < 0 || z < 0 || x >= nx as isize || y >= ny as isize || z >= nz as isize {
            false
        } else {
            occupied[((x as usize * ny + y as usize) * nz) + z as usize]
        }
    };
    let mut edges = Vec::new();

    // X-directed edges inspect the four Y/Z cells around each line.
    for y in 0..=ny {
        for z in 0..=nz {
            let offsets = (0..nx)
                .map(|x| {
                    boundary_offset([
                        cell(x as isize, y as isize - 1, z as isize - 1),
                        cell(x as isize, y as isize - 1, z as isize),
                        cell(x as isize, y as isize, z as isize - 1),
                        cell(x as isize, y as isize, z as isize),
                    ])
                    .map(|(dy, dz)| Vec3::new(0.0, dy, dz))
                })
                .collect();
            append_edge_runs(&xs, offsets, |x| Vec3::new(x, ys[y], zs[z]), &mut edges);
        }
    }

    // Y-directed edges inspect the four X/Z cells around each line.
    for x in 0..=nx {
        for z in 0..=nz {
            let offsets = (0..ny)
                .map(|y| {
                    boundary_offset([
                        cell(x as isize - 1, y as isize, z as isize - 1),
                        cell(x as isize - 1, y as isize, z as isize),
                        cell(x as isize, y as isize, z as isize - 1),
                        cell(x as isize, y as isize, z as isize),
                    ])
                    .map(|(dx, dz)| Vec3::new(dx, 0.0, dz))
                })
                .collect();
            append_edge_runs(&ys, offsets, |y| Vec3::new(xs[x], y, zs[z]), &mut edges);
        }
    }

    // Z-directed edges inspect the four X/Y cells around each line.
    for x in 0..=nx {
        for y in 0..=ny {
            let offsets = (0..nz)
                .map(|z| {
                    boundary_offset([
                        cell(x as isize - 1, y as isize - 1, z as isize),
                        cell(x as isize - 1, y as isize, z as isize),
                        cell(x as isize, y as isize - 1, z as isize),
                        cell(x as isize, y as isize, z as isize),
                    ])
                    .map(|(dx, dy)| Vec3::new(dx, dy, 0.0))
                })
                .collect();
            append_edge_runs(&zs, offsets, |z| Vec3::new(xs[x], ys[y], z), &mut edges);
        }
    }
    edges
}

fn axis_coordinates(boxes: &[Aabb], axis: usize) -> Vec<f32> {
    const COORDINATE_EPSILON: f32 = 1.0e-6;
    let mut coordinates = boxes
        .iter()
        .flat_map(|bounds| [bounds.min[axis], bounds.max[axis]])
        .filter(|coordinate| coordinate.is_finite())
        .collect::<Vec<_>>();
    coordinates.sort_by(f32::total_cmp);
    coordinates.dedup_by(|left, right| (*left - *right).abs() <= COORDINATE_EPSILON);
    coordinates
}

fn coordinate_index(coordinates: &[f32], value: f32) -> Option<usize> {
    const COORDINATE_EPSILON: f32 = 1.0e-6;
    coordinates
        .iter()
        .position(|coordinate| (*coordinate - value).abs() <= COORDINATE_EPSILON)
}

/// Returns the two fixed-axis expansion components when a line is an edge of
/// the union. Two adjacent occupied quadrants are one flat surface and are
/// therefore deliberately omitted.
fn boundary_offset(occupied: [bool; 4]) -> Option<(f32, f32)> {
    const SIDES: [(f32, f32); 4] = [(-1.0, -1.0), (-1.0, 1.0), (1.0, -1.0), (1.0, 1.0)];
    let occupied_indices = occupied
        .iter()
        .enumerate()
        .filter_map(|(index, occupied)| occupied.then_some(index))
        .collect::<Vec<_>>();
    match occupied_indices.as_slice() {
        [only] => {
            let (a, b) = SIDES[*only];
            Some((-a, -b))
        }
        [first, second] => {
            let first = SIDES[*first];
            let second = SIDES[*second];
            (first.0 != second.0 && first.1 != second.1).then_some((0.0, 0.0))
        }
        [_, _, _] => {
            let empty = occupied.iter().position(|occupied| !occupied).unwrap();
            Some(SIDES[empty])
        }
        _ => None,
    }
}

fn append_edge_runs(
    coordinates: &[f32],
    offsets: Vec<Option<Vec3>>,
    point: impl Fn(f32) -> Vec3,
    output: &mut Vec<BlockShapeEdge>,
) {
    let mut run_start = 0;
    let mut run_offset = None;
    for index in 0..=offsets.len() {
        let next_offset = offsets.get(index).copied().flatten();
        if next_offset == run_offset {
            continue;
        }
        if let Some(expansion_direction) = run_offset {
            output.push(BlockShapeEdge {
                start: point(coordinates[run_start]),
                end: point(coordinates[index]),
                expansion_direction,
            });
        }
        run_start = index;
        run_offset = next_offset;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_cube_has_only_twelve_boundary_edges() {
        assert_eq!(BlockShape::full_cube().boundary_edges().len(), 12);
    }

    #[test]
    fn eight_tiled_boxes_collapse_to_one_outer_cube() {
        let mut boxes = Vec::new();
        for x in 0..2 {
            for y in 0..2 {
                for z in 0..2 {
                    let min = Vec3::new(x as f32, y as f32, z as f32) * 0.5;
                    boxes.push(Aabb {
                        min,
                        max: min + Vec3::splat(0.5),
                    });
                }
            }
        }
        let shape = BlockShape::new(boxes);

        assert_eq!(shape.boundary_edges().len(), 12);
        assert!(
            shape.boundary_edges().iter().all(|edge| {
                (0..3).all(|axis| edge.start[axis] != 0.5 || edge.end[axis] != 0.5)
            })
        );
    }

    #[test]
    fn stair_union_keeps_concave_and_convex_boundary_edges() {
        let shape = BlockShape::new([
            Aabb {
                min: Vec3::ZERO,
                max: Vec3::new(1.0, 0.5, 1.0),
            },
            Aabb {
                min: Vec3::new(0.5, 0.5, 0.0),
                max: Vec3::ONE,
            },
        ]);

        assert_eq!(shape.boundary_edges().len(), 18);
    }
}
