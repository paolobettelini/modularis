use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct BubbleCoralBlockBlock;

impl Block for BubbleCoralBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:bubble-coral-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for BubbleCoralBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-bubble-coral-block:block/bubble_coral_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = BubbleCoralBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BubbleCoralBlockBlock::RENDER;

pub struct BlockBubbleCoralBlockMod;

impl BlockBubbleCoralBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
