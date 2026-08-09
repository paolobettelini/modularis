use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct SmoothQuartzBlock;

impl Block for SmoothQuartzBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:smooth-quartz",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for SmoothQuartzBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-smooth-quartz:block/smooth_quartz"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = SmoothQuartzBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = SmoothQuartzBlock::RENDER;

pub struct BlockSmoothQuartzMod;

impl BlockSmoothQuartzMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
