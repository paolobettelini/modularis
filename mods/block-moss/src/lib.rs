use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, BlockTextures, RenderShape};
use tokio::task::JoinHandle;

pub struct MossBlock;
impl Block for MossBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:moss",
        is_air: false,
        solid: true,
        opaque: true,
    };
}
impl BlockRender for MossBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Cube,
        textures: Some(BlockTextures::Uniform("block-moss/moss.png")),
    };
}
pub const BLOCK_INFO: BlockInfo = MossBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = MossBlock::RENDER;
pub struct BlockMossMod;
impl BlockMossMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
