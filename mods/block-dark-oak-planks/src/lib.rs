use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct DarkOakPlanksBlock;

impl Block for DarkOakPlanksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:dark-oak-planks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for DarkOakPlanksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-dark-oak-planks:block/dark_oak_planks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = DarkOakPlanksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = DarkOakPlanksBlock::RENDER;

pub struct BlockDarkOakPlanksMod;

impl BlockDarkOakPlanksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
