use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, BlockTextures, RenderShape};
use tokio::task::JoinHandle;

pub struct OakLeavesBlock;

impl Block for OakLeavesBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:oak-leaves",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for OakLeavesBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Cube,
        textures: Some(BlockTextures::Uniform("block-oak-leaves/oak_leaves.png")),
    };
}

pub const BLOCK_INFO: BlockInfo = OakLeavesBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = OakLeavesBlock::RENDER;

pub struct BlockOakLeavesMod;

impl BlockOakLeavesMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
