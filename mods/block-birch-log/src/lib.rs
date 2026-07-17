use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, BlockTextures, RenderShape};
use tokio::task::JoinHandle;

pub struct BirchLogBlock;

impl Block for BirchLogBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:birch-log",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for BirchLogBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Cube,
        textures: Some(BlockTextures::PerFace {
            east: "block-birch-log/birch_log_side.png",
            west: "block-birch-log/birch_log_side.png",
            top: "block-birch-log/birch_log_top.png",
            bottom: "block-birch-log/birch_log_top.png",
            south: "block-birch-log/birch_log_side.png",
            north: "block-birch-log/birch_log_side.png",
        }),
    };
}

pub const BLOCK_INFO: BlockInfo = BirchLogBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BirchLogBlock::RENDER;

pub struct BlockBirchLogMod;

impl BlockBirchLogMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
