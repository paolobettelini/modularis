use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, BlockTextures, RenderShape};
use tokio::task::JoinHandle;

pub struct DiamondBlockBlock;

impl Block for DiamondBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:diamond-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for DiamondBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Cube,
        textures: Some(BlockTextures::Uniform(
            "block-diamond-block/diamond_block.png",
        )),
    };
}

pub const BLOCK_INFO: BlockInfo = DiamondBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = DiamondBlockBlock::RENDER;

pub struct BlockDiamondBlockMod;

impl BlockDiamondBlockMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
