use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct BrainCoralBlockBlock;

impl Block for BrainCoralBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:brain-coral-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for BrainCoralBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-brain-coral-block:block/brain_coral_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = BrainCoralBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BrainCoralBlockBlock::RENDER;

pub struct BlockBrainCoralBlockMod;

impl BlockBrainCoralBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
