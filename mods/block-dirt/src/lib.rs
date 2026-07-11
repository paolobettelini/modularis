use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, BlockTextures, RenderShape};
use tokio::task::JoinHandle;

pub struct DirtBlock;

impl Block for DirtBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:dirt",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for DirtBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Cube,
        textures: Some(BlockTextures::Uniform("block-dirt/dirt.png")),
    };
}

pub const BLOCK_INFO: BlockInfo = DirtBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = DirtBlock::RENDER;

pub struct BlockDirtMod;

impl BlockDirtMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
