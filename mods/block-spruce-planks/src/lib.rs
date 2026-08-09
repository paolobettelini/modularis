use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct SprucePlanksBlock;

impl Block for SprucePlanksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:spruce-planks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for SprucePlanksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-spruce-planks:block/spruce_planks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = SprucePlanksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = SprucePlanksBlock::RENDER;

pub struct BlockSprucePlanksMod;

impl BlockSprucePlanksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
