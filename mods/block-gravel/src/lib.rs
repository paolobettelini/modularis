use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, BlockTextures, RenderShape};
use tokio::task::JoinHandle;

pub struct GravelBlock;

impl Block for GravelBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:gravel",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for GravelBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Cube,
        textures: Some(BlockTextures::Uniform("block-gravel/gravel.png")),
    };
}

pub const BLOCK_INFO: BlockInfo = GravelBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = GravelBlock::RENDER;

pub struct BlockGravelMod;

impl BlockGravelMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
