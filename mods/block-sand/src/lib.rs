use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, BlockTextures, RenderShape};
use tokio::task::JoinHandle;

pub struct SandBlock;

impl Block for SandBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:sand",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for SandBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Cube,
        textures: Some(BlockTextures::Uniform("block-sand/sand.png")),
    };
}

pub const BLOCK_INFO: BlockInfo = SandBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = SandBlock::RENDER;

pub struct BlockSandMod;

impl BlockSandMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
