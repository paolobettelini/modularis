use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct TubeCoralBlockBlock;

impl Block for TubeCoralBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:tube-coral-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for TubeCoralBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-tube-coral-block:block/tube_coral_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = TubeCoralBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = TubeCoralBlockBlock::RENDER;

pub struct BlockTubeCoralBlockMod;

impl BlockTubeCoralBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
