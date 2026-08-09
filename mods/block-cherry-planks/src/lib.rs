use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct CherryPlanksBlock;

impl Block for CherryPlanksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:cherry-planks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for CherryPlanksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-cherry-planks:block/cherry_planks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = CherryPlanksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CherryPlanksBlock::RENDER;

pub struct BlockCherryPlanksMod;

impl BlockCherryPlanksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
