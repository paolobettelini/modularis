use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct CherryLogBlock;

impl Block for CherryLogBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:cherry-log",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for CherryLogBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-cherry-log:block/cherry_log"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = CherryLogBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CherryLogBlock::RENDER;

pub struct BlockCherryLogMod;

impl BlockCherryLogMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
