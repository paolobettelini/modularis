use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct StrippedDarkOakLogBlock;

impl Block for StrippedDarkOakLogBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:stripped-dark-oak-log",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for StrippedDarkOakLogBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-stripped-dark-oak-log:block/stripped_dark_oak_log"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = StrippedDarkOakLogBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = StrippedDarkOakLogBlock::RENDER;

pub struct BlockStrippedDarkOakLogMod;

impl BlockStrippedDarkOakLogMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
