use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct StrippedSpruceLogBlock;

impl Block for StrippedSpruceLogBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:stripped-spruce-log",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for StrippedSpruceLogBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-stripped-spruce-log:block/stripped_spruce_log"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = StrippedSpruceLogBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = StrippedSpruceLogBlock::RENDER;

pub struct BlockStrippedSpruceLogMod;

impl BlockStrippedSpruceLogMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
