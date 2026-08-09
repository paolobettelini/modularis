use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct OakPlanksBlock;

impl Block for OakPlanksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:oak-planks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for OakPlanksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-oak-planks:block/oak_planks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = OakPlanksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = OakPlanksBlock::RENDER;

pub struct BlockOakPlanksMod;

impl BlockOakPlanksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
