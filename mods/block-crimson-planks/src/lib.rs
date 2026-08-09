use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct CrimsonPlanksBlock;

impl Block for CrimsonPlanksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:crimson-planks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for CrimsonPlanksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-crimson-planks:block/crimson_planks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = CrimsonPlanksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CrimsonPlanksBlock::RENDER;

pub struct BlockCrimsonPlanksMod;

impl BlockCrimsonPlanksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
