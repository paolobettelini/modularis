use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, BlockTextures, RenderShape};
use tokio::task::JoinHandle;

pub struct BlackstoneBlock;
impl Block for BlackstoneBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:blackstone",
        is_air: false,
        solid: true,
        opaque: true,
    };
}
impl BlockRender for BlackstoneBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Cube,
        textures: Some(BlockTextures::PerFace {
            east: "block-blackstone/blackstone_side.png",
            west: "block-blackstone/blackstone_side.png",
            top: "block-blackstone/blackstone_top.png",
            bottom: "block-blackstone/blackstone_top.png",
            south: "block-blackstone/blackstone_side.png",
            north: "block-blackstone/blackstone_side.png",
        }),
    };
}
pub const BLOCK_INFO: BlockInfo = BlackstoneBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BlackstoneBlock::RENDER;
pub struct BlockBlackstoneMod;
impl BlockBlackstoneMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
