use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, BlockTextures, RenderShape};
use tokio::task::JoinHandle;

pub struct GrassBlock;

impl Block for GrassBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:grass",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for GrassBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Cube,
        textures: Some(BlockTextures::Uniform("block-grass/grass.png")),
    };
}

pub const BLOCK_INFO: BlockInfo = GrassBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = GrassBlock::RENDER;

pub struct BlockGrassMod;

impl BlockGrassMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
