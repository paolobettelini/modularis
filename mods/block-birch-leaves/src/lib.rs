use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, BlockTextures, RenderShape};
use tokio::task::JoinHandle;

pub struct BirchLeavesBlock;

impl Block for BirchLeavesBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:birch-leaves",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for BirchLeavesBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Cube,
        textures: Some(BlockTextures::Uniform(
            "block-birch-leaves/birch_leaves.png",
        )),
    };
}

pub const BLOCK_INFO: BlockInfo = BirchLeavesBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BirchLeavesBlock::RENDER;

pub struct BlockBirchLeavesMod;

impl BlockBirchLeavesMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
