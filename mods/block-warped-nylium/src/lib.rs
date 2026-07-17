use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, BlockTextures, RenderShape};
use tokio::task::JoinHandle;

pub struct WarpedNyliumBlock;
impl Block for WarpedNyliumBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:warped-nylium",
        is_air: false,
        solid: true,
        opaque: true,
    };
}
impl BlockRender for WarpedNyliumBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Cube,
        textures: Some(BlockTextures::PerFace {
            east: "block-warped-nylium/warped_nylium_side.png",
            west: "block-warped-nylium/warped_nylium_side.png",
            top: "block-warped-nylium/warped_nylium_top.png",
            bottom: "block-warped-nylium/warped_nylium_side.png",
            south: "block-warped-nylium/warped_nylium_side.png",
            north: "block-warped-nylium/warped_nylium_side.png",
        }),
    };
}
pub const BLOCK_INFO: BlockInfo = WarpedNyliumBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = WarpedNyliumBlock::RENDER;
pub struct BlockWarpedNyliumMod;
impl BlockWarpedNyliumMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
