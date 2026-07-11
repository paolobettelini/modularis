use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, BlockTextures, RenderShape};
use tokio::task::JoinHandle;

pub struct ObsidianBlock;

impl Block for ObsidianBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:obsidian",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for ObsidianBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Cube,
        textures: Some(BlockTextures::Uniform("block-obsidian/obsidian.png")),
    };
}

pub const BLOCK_INFO: BlockInfo = ObsidianBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = ObsidianBlock::RENDER;

pub struct BlockObsidianMod;

impl BlockObsidianMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
