use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct StrippedOakLogBlock;

impl Block for StrippedOakLogBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:stripped-oak-log",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for StrippedOakLogBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-stripped-oak-log:block/stripped_oak_log"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = StrippedOakLogBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = StrippedOakLogBlock::RENDER;

pub struct BlockStrippedOakLogMod;

impl BlockStrippedOakLogMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
