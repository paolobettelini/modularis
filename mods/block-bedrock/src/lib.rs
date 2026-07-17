use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
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
        shape: RenderShape::Model,
        model: Some("block-bedrock:block/bedrock"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = BedrockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BedrockBlock::RENDER;

pub struct BlockBedrockMod;

impl BlockBedrockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
