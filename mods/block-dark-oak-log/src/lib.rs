use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct DarkOakLogBlock;

impl Block for DarkOakLogBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:dark-oak-log",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for DarkOakLogBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-dark-oak-log:block/dark_oak_log"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = DarkOakLogBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = DarkOakLogBlock::RENDER;

pub struct BlockDarkOakLogMod;

impl BlockDarkOakLogMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
