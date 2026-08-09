use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct SmoothBasaltBlock;

impl Block for SmoothBasaltBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:smooth-basalt",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for SmoothBasaltBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-smooth-basalt:block/smooth_basalt"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = SmoothBasaltBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = SmoothBasaltBlock::RENDER;

pub struct BlockSmoothBasaltMod;

impl BlockSmoothBasaltMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
