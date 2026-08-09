use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct DeadHornCoralBlockBlock;

impl Block for DeadHornCoralBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:dead-horn-coral-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for DeadHornCoralBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-dead-horn-coral-block:block/dead_horn_coral_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = DeadHornCoralBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = DeadHornCoralBlockBlock::RENDER;

pub struct BlockDeadHornCoralBlockMod;

impl BlockDeadHornCoralBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
