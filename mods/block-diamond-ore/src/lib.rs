use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, BlockTextures, RenderShape};
use tokio::task::JoinHandle;

pub struct DiamondOreBlock;

impl Block for DiamondOreBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:diamond-ore",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for DiamondOreBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Cube,
        textures: Some(BlockTextures::Uniform("block-diamond-ore/diamond_ore.png")),
    };
}

pub const BLOCK_INFO: BlockInfo = DiamondOreBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = DiamondOreBlock::RENDER;

pub struct BlockDiamondOreMod;

impl BlockDiamondOreMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
