use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, BlockTextures, RenderShape};
use tokio::task::JoinHandle;

pub struct SoulSandBlock;
impl Block for SoulSandBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:soul-sand",
        is_air: false,
        solid: true,
        opaque: true,
    };
}
impl BlockRender for SoulSandBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Cube,
        textures: Some(BlockTextures::Uniform("block-soul-sand/soul_sand.png")),
    };
}
pub const BLOCK_INFO: BlockInfo = SoulSandBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = SoulSandBlock::RENDER;
pub struct BlockSoulSandMod;
impl BlockSoulSandMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
