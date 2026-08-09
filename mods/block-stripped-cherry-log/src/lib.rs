use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct StrippedCherryLogBlock;

impl Block for StrippedCherryLogBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:stripped-cherry-log",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for StrippedCherryLogBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-stripped-cherry-log:block/stripped_cherry_log"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = StrippedCherryLogBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = StrippedCherryLogBlock::RENDER;

pub struct BlockStrippedCherryLogMod;

impl BlockStrippedCherryLogMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
