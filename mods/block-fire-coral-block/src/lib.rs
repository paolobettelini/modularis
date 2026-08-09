use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct FireCoralBlockBlock;

impl Block for FireCoralBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:fire-coral-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for FireCoralBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-fire-coral-block:block/fire_coral_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = FireCoralBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = FireCoralBlockBlock::RENDER;

pub struct BlockFireCoralBlockMod;

impl BlockFireCoralBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
