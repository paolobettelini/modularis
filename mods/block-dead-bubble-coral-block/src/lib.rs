use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct DeadBubbleCoralBlockBlock;

impl Block for DeadBubbleCoralBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:dead-bubble-coral-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for DeadBubbleCoralBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-dead-bubble-coral-block:block/dead_bubble_coral_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = DeadBubbleCoralBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = DeadBubbleCoralBlockBlock::RENDER;

pub struct BlockDeadBubbleCoralBlockMod;

impl BlockDeadBubbleCoralBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
