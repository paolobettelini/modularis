use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, BlockTextures, RenderShape};
use tokio::task::JoinHandle;

pub struct CrimsonNyliumBlock;
impl Block for CrimsonNyliumBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:crimson-nylium",
        is_air: false,
        solid: true,
        opaque: true,
    };
}
impl BlockRender for CrimsonNyliumBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Cube,
        textures: Some(BlockTextures::PerFace {
            east: "block-crimson-nylium/crimson_nylium_side.png",
            west: "block-crimson-nylium/crimson_nylium_side.png",
            top: "block-crimson-nylium/crimson_nylium_top.png",
            bottom: "block-crimson-nylium/crimson_nylium_side.png",
            south: "block-crimson-nylium/crimson_nylium_side.png",
            north: "block-crimson-nylium/crimson_nylium_side.png",
        }),
    };
}
pub const BLOCK_INFO: BlockInfo = CrimsonNyliumBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CrimsonNyliumBlock::RENDER;
pub struct BlockCrimsonNyliumMod;
impl BlockCrimsonNyliumMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
