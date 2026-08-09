use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct StrippedMangroveLogBlock;

impl Block for StrippedMangroveLogBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:stripped-mangrove-log",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for StrippedMangroveLogBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-stripped-mangrove-log:block/stripped_mangrove_log"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = StrippedMangroveLogBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = StrippedMangroveLogBlock::RENDER;

pub struct BlockStrippedMangroveLogMod;

impl BlockStrippedMangroveLogMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
