use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, BlockTextures, RenderShape};
use tokio::task::JoinHandle;

pub struct BasaltBlock;
impl Block for BasaltBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:basalt",
        is_air: false,
        solid: true,
        opaque: true,
    };
}
impl BlockRender for BasaltBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Cube,
        textures: Some(BlockTextures::PerFace {
            east: "block-basalt/basalt_side.png",
            west: "block-basalt/basalt_side.png",
            top: "block-basalt/basalt_top.png",
            bottom: "block-basalt/basalt_top.png",
            south: "block-basalt/basalt_side.png",
            north: "block-basalt/basalt_side.png",
        }),
    };
}
pub const BLOCK_INFO: BlockInfo = BasaltBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BasaltBlock::RENDER;
pub struct BlockBasaltMod;
impl BlockBasaltMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
