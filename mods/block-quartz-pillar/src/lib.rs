use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct QuartzPillarBlock;

impl Block for QuartzPillarBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:quartz-pillar",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for QuartzPillarBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-quartz-pillar:block/quartz_pillar"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = QuartzPillarBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = QuartzPillarBlock::RENDER;

pub struct BlockQuartzPillarMod;

impl BlockQuartzPillarMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
