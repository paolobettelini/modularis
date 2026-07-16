use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, BlockTextures, RenderShape};
use tokio::task::JoinHandle;

pub struct OakLogBlock;

impl Block for OakLogBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:oak-log",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for OakLogBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Cube,
        textures: Some(BlockTextures::PerFace {
            east: "block-oak-log/oak_log_side.png",
            west: "block-oak-log/oak_log_side.png",
            top: "block-oak-log/oak_log_top.png",
            bottom: "block-oak-log/oak_log_top.png",
            south: "block-oak-log/oak_log_side.png",
            north: "block-oak-log/oak_log_side.png",
        }),
    };
}

pub const BLOCK_INFO: BlockInfo = OakLogBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = OakLogBlock::RENDER;

pub struct BlockOakLogMod;

impl BlockOakLogMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
