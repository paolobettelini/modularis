use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct JunglePlanksBlock;

impl Block for JunglePlanksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:jungle-planks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for JunglePlanksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-jungle-planks:block/jungle_planks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = JunglePlanksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = JunglePlanksBlock::RENDER;

pub struct BlockJunglePlanksMod;

impl BlockJunglePlanksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
