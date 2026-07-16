use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, BlockTextures, RenderShape};
use tokio::task::JoinHandle;

pub struct SnowBlock;

impl Block for SnowBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:snow",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for SnowBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Cube,
        textures: Some(BlockTextures::Uniform("block-snow/snow.png")),
    };
}

pub const BLOCK_INFO: BlockInfo = SnowBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = SnowBlock::RENDER;

pub struct BlockSnowMod;

impl BlockSnowMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
