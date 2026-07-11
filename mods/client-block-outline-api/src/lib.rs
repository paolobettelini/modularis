use bevy::prelude::*;
use voxel_math_api::BlockPos;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlockOutlineStyle {
    pub color: [f32; 4],
    /// Distance in world units between the block faces and the outline.
    pub expansion: f32,
}

impl Default for BlockOutlineStyle {
    fn default() -> Self {
        Self {
            color: [0.05, 0.05, 0.05, 1.0],
            expansion: 0.002,
        }
    }
}

#[derive(Message, Debug, Clone, PartialEq)]
pub struct SetClientBlockOutline {
    /// Stable owner key. Different mods can maintain independent outlines.
    pub owner: String,
    /// `None` removes the outline owned by `owner`.
    pub block: Option<BlockPos>,
    pub style: BlockOutlineStyle,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClientBlockOutlineSet {
    Collect,
    Apply,
    Draw,
}

pub trait ClientBlockOutlineApi: Send + Sync + 'static {}
