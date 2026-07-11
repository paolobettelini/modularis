use serde::{Deserialize, Serialize};
use voxel_math_api::BlockPos;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PortalAxis {
    X,
    Z,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PortalFrame {
    /// Minimum outer-frame corner. Corners are optional; the occupied span is 4x5.
    pub origin: BlockPos,
    pub axis: PortalAxis,
}

impl PortalFrame {
    pub fn block_at(self, horizontal: i32, vertical: i32) -> BlockPos {
        match self.axis {
            PortalAxis::X => BlockPos::new(
                self.origin.x + horizontal,
                self.origin.y + vertical,
                self.origin.z,
            ),
            PortalAxis::Z => BlockPos::new(
                self.origin.x,
                self.origin.y + vertical,
                self.origin.z + horizontal,
            ),
        }
    }

    /// Blocks required by the vanilla 4x5 frame. The four corners are omitted.
    pub fn required_frame_blocks(self) -> impl Iterator<Item = BlockPos> {
        (0..5).flat_map(move |vertical| {
            (0..4).filter_map(move |horizontal| {
                let side = (horizontal == 0 || horizontal == 3) && (1..4).contains(&vertical);
                let cap = (vertical == 0 || vertical == 4) && (1..3).contains(&horizontal);
                (side || cap).then_some(self.block_at(horizontal, vertical))
            })
        })
    }

    pub fn interior_blocks(self) -> impl Iterator<Item = BlockPos> {
        (1..4).flat_map(move |vertical| {
            (1..3).map(move |horizontal| self.block_at(horizontal, vertical))
        })
    }

    pub fn touches_chunk(self, chunk: voxel_math_api::ChunkPos) -> bool {
        self.required_frame_blocks()
            .chain(self.interior_blocks())
            .any(|position| position.chunk() == chunk)
    }

    pub fn contains_player(self, position: [f32; 3], radius: f32, height: f32) -> bool {
        let x = position[0];
        let feet = position[1];
        let head = feet + height;
        let z = position[2];
        let vertical_overlap =
            head > self.origin.y as f32 + 1.0 && feet < self.origin.y as f32 + 4.0;
        if !vertical_overlap {
            return false;
        }
        match self.axis {
            PortalAxis::X => {
                x + radius > self.origin.x as f32 + 1.0
                    && x - radius < self.origin.x as f32 + 3.0
                    && (z - (self.origin.z as f32 + 0.5)).abs() <= radius + 0.2
            }
            PortalAxis::Z => {
                z + radius > self.origin.z as f32 + 1.0
                    && z - radius < self.origin.z as f32 + 3.0
                    && (x - (self.origin.x as f32 + 0.5)).abs() <= radius + 0.2
            }
        }
    }

    pub fn safe_position_beside(self) -> [f32; 3] {
        match self.axis {
            PortalAxis::X => [
                self.origin.x as f32 + 2.0,
                self.origin.y as f32 + 1.0,
                self.origin.z as f32 + 2.0,
            ],
            PortalAxis::Z => [
                self.origin.x as f32 + 2.0,
                self.origin.y as f32 + 1.0,
                self.origin.z as f32 + 2.0,
            ],
        }
    }
}

pub fn find_ignitable_frame(
    interior_face: BlockPos,
    mut is_frame_block: impl FnMut(BlockPos) -> bool,
    mut is_empty: impl FnMut(BlockPos) -> bool,
) -> Option<PortalFrame> {
    for axis in [PortalAxis::X, PortalAxis::Z] {
        for vertical in 1..4 {
            for horizontal in 1..3 {
                let origin = match axis {
                    PortalAxis::X => BlockPos::new(
                        interior_face.x - horizontal,
                        interior_face.y - vertical,
                        interior_face.z,
                    ),
                    PortalAxis::Z => BlockPos::new(
                        interior_face.x,
                        interior_face.y - vertical,
                        interior_face.z - horizontal,
                    ),
                };
                let frame = PortalFrame { origin, axis };
                if frame.required_frame_blocks().all(&mut is_frame_block)
                    && frame.interior_blocks().all(&mut is_empty)
                {
                    return Some(frame);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn frame_is_valid_without_corners() {
        let frame = PortalFrame {
            origin: BlockPos::new(10, 2, 4),
            axis: PortalAxis::X,
        };
        assert_eq!(frame.required_frame_blocks().count(), 10);
        let frame_blocks = frame.required_frame_blocks().collect::<HashSet<_>>();
        let interior = frame.interior_blocks().collect::<HashSet<_>>();
        assert_eq!(
            find_ignitable_frame(
                BlockPos::new(11, 3, 4),
                |position| frame_blocks.contains(&position),
                |position| interior.contains(&position),
            ),
            Some(frame)
        );
    }
}
