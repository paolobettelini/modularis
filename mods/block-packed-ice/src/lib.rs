use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, BlockTextures, RenderShape};
use tokio::task::JoinHandle;

pub struct PackedIceBlock;

impl Block for PackedIceBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:packed-ice",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PackedIceBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Cube,
        textures: Some(BlockTextures::Uniform("block-packed-ice/packed_ice.png")),
    };
}

pub const BLOCK_INFO: BlockInfo = PackedIceBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PackedIceBlock::RENDER;

pub struct BlockPackedIceMod;

impl BlockPackedIceMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
