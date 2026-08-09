use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct WarpedPlanksBlock;

impl Block for WarpedPlanksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:warped-planks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for WarpedPlanksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-warped-planks:block/warped_planks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = WarpedPlanksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = WarpedPlanksBlock::RENDER;

pub struct BlockWarpedPlanksMod;

impl BlockWarpedPlanksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
