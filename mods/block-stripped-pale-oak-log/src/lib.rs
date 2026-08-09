use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct StrippedPaleOakLogBlock;

impl Block for StrippedPaleOakLogBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:stripped-pale-oak-log",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for StrippedPaleOakLogBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-stripped-pale-oak-log:block/stripped_pale_oak_log"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = StrippedPaleOakLogBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = StrippedPaleOakLogBlock::RENDER;

pub struct BlockStrippedPaleOakLogMod;

impl BlockStrippedPaleOakLogMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
