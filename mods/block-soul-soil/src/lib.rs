use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, BlockTextures, RenderShape};
use tokio::task::JoinHandle;

pub struct SoulSoilBlock;
impl Block for SoulSoilBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:soul-soil",
        is_air: false,
        solid: true,
        opaque: true,
    };
}
impl BlockRender for SoulSoilBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Cube,
        textures: Some(BlockTextures::Uniform("block-soul-soil/soul_soil.png")),
    };
}
pub const BLOCK_INFO: BlockInfo = SoulSoilBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = SoulSoilBlock::RENDER;
pub struct BlockSoulSoilMod;
impl BlockSoulSoilMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
