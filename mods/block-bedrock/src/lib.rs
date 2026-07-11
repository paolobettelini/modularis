use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, BlockTextures, RenderShape};
use tokio::task::JoinHandle;

pub struct BedrockBlock;

impl Block for BedrockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:bedrock",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for BedrockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Cube,
        textures: Some(BlockTextures::Uniform("block-bedrock/bedrock.png")),
    };
}

pub const BLOCK_INFO: BlockInfo = BedrockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BedrockBlock::RENDER;

pub struct BlockBedrockMod;

impl BlockBedrockMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
