use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, BlockTextures, RenderShape};
use tokio::task::JoinHandle;

pub struct TerracottaBlock;
impl Block for TerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}
impl BlockRender for TerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Cube,
        textures: Some(BlockTextures::Uniform("block-terracotta/terracotta.png")),
    };
}
pub const BLOCK_INFO: BlockInfo = TerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = TerracottaBlock::RENDER;
pub struct BlockTerracottaMod;
impl BlockTerracottaMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
