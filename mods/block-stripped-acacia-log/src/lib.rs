use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct StrippedAcaciaLogBlock;

impl Block for StrippedAcaciaLogBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:stripped-acacia-log",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for StrippedAcaciaLogBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-stripped-acacia-log:block/stripped_acacia_log"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = StrippedAcaciaLogBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = StrippedAcaciaLogBlock::RENDER;

pub struct BlockStrippedAcaciaLogMod;

impl BlockStrippedAcaciaLogMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
