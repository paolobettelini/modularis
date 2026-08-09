use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct QuartzBricksBlock;

impl Block for QuartzBricksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:quartz-bricks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for QuartzBricksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-quartz-bricks:block/quartz_bricks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = QuartzBricksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = QuartzBricksBlock::RENDER;

pub struct BlockQuartzBricksMod;

impl BlockQuartzBricksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
