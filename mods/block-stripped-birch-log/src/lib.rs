use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct StrippedBirchLogBlock;

impl Block for StrippedBirchLogBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:stripped-birch-log",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for StrippedBirchLogBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-stripped-birch-log:block/stripped_birch_log"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = StrippedBirchLogBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = StrippedBirchLogBlock::RENDER;

pub struct BlockStrippedBirchLogMod;

impl BlockStrippedBirchLogMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
