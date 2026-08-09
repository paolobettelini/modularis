use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PaleOakPlanksBlock;

impl Block for PaleOakPlanksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:pale-oak-planks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PaleOakPlanksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-pale-oak-planks:block/pale_oak_planks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PaleOakPlanksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PaleOakPlanksBlock::RENDER;

pub struct BlockPaleOakPlanksMod;

impl BlockPaleOakPlanksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
