use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct QuartzBlockBlock;

impl Block for QuartzBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:quartz-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for QuartzBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-quartz-block:block/quartz_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = QuartzBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = QuartzBlockBlock::RENDER;

pub struct BlockQuartzBlockMod;

impl BlockQuartzBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
