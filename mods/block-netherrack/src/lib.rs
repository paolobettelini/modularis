use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, BlockTextures, RenderShape};
use tokio::task::JoinHandle;

pub struct NetherrackBlock;

impl Block for NetherrackBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:netherrack",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for NetherrackBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Cube,
        textures: Some(BlockTextures::Uniform("block-netherrack/netherrack.png")),
    };
}

pub const BLOCK_INFO: BlockInfo = NetherrackBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = NetherrackBlock::RENDER;

pub struct BlockNetherrackMod;

impl BlockNetherrackMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
