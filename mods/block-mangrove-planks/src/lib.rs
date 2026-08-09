use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct MangrovePlanksBlock;

impl Block for MangrovePlanksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:mangrove-planks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for MangrovePlanksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-mangrove-planks:block/mangrove_planks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = MangrovePlanksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = MangrovePlanksBlock::RENDER;

pub struct BlockMangrovePlanksMod;

impl BlockMangrovePlanksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
