use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct AcaciaLogBlock;

impl Block for AcaciaLogBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:acacia-log",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for AcaciaLogBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-acacia-log:block/acacia_log"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = AcaciaLogBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = AcaciaLogBlock::RENDER;

pub struct BlockAcaciaLogMod;

impl BlockAcaciaLogMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
