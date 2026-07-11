use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct AirBlock;

impl Block for AirBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:air",
        is_air: true,
        solid: false,
        opaque: false,
    };
}

impl BlockRender for AirBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Invisible,
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = AirBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = AirBlock::RENDER;

pub struct BlockAirMod;

impl BlockAirMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
