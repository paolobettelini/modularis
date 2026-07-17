use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, BlockTextures, RenderShape};
use tokio::task::JoinHandle;

pub struct RedSandBlock;

impl Block for RedSandBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:red-sand",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for RedSandBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Cube,
        textures: Some(BlockTextures::Uniform("block-red-sand/red_sand.png")),
    };
}

pub const BLOCK_INFO: BlockInfo = RedSandBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = RedSandBlock::RENDER;

pub struct BlockRedSandMod;
impl BlockRedSandMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
