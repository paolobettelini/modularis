use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct DeadTubeCoralBlockBlock;

impl Block for DeadTubeCoralBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:dead-tube-coral-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for DeadTubeCoralBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-dead-tube-coral-block:block/dead_tube_coral_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = DeadTubeCoralBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = DeadTubeCoralBlockBlock::RENDER;

pub struct BlockDeadTubeCoralBlockMod;

impl BlockDeadTubeCoralBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
