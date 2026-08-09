use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct DeadFireCoralBlockBlock;

impl Block for DeadFireCoralBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:dead-fire-coral-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for DeadFireCoralBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-dead-fire-coral-block:block/dead_fire_coral_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = DeadFireCoralBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = DeadFireCoralBlockBlock::RENDER;

pub struct BlockDeadFireCoralBlockMod;

impl BlockDeadFireCoralBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
