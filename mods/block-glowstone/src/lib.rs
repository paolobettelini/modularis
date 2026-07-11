use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, BlockTextures, RenderShape};
use tokio::task::JoinHandle;

pub struct GlowstoneBlock;

impl Block for GlowstoneBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:glowstone",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for GlowstoneBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Cube,
        textures: Some(BlockTextures::Uniform("block-glowstone/glowstone.png")),
    };
}

pub const BLOCK_INFO: BlockInfo = GlowstoneBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = GlowstoneBlock::RENDER;

pub struct BlockGlowstoneMod;

impl BlockGlowstoneMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
