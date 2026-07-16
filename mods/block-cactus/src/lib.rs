use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, BlockTextures, RenderShape};
use tokio::task::JoinHandle;

pub struct CactusBlock;

impl Block for CactusBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:cactus",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for CactusBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Cube,
        textures: Some(BlockTextures::PerFace {
            east: "block-cactus/cactus_side.png",
            west: "block-cactus/cactus_side.png",
            top: "block-cactus/cactus_top.png",
            bottom: "block-cactus/cactus_bottom.png",
            south: "block-cactus/cactus_side.png",
            north: "block-cactus/cactus_side.png",
        }),
    };
}

pub const BLOCK_INFO: BlockInfo = CactusBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CactusBlock::RENDER;

pub struct BlockCactusMod;

impl BlockCactusMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
