use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct MangroveLogBlock;

impl Block for MangroveLogBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:mangrove-log",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for MangroveLogBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-mangrove-log:block/mangrove_log"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = MangroveLogBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = MangroveLogBlock::RENDER;

pub struct BlockMangroveLogMod;

impl BlockMangroveLogMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
