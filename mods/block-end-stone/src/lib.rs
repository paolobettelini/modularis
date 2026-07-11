use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, BlockTextures, RenderShape};
use tokio::task::JoinHandle;

pub struct EndStoneBlock;

impl Block for EndStoneBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:end-stone",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for EndStoneBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Cube,
        textures: Some(BlockTextures::Uniform("block-end-stone/end_stone.png")),
    };
}

pub const BLOCK_INFO: BlockInfo = EndStoneBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = EndStoneBlock::RENDER;

pub struct BlockEndStoneMod;

impl BlockEndStoneMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
