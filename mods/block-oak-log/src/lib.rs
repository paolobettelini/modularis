use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct OakLogBlock;

impl Block for OakLogBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:oak-log",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for OakLogBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-oak-log:block/oak_log"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = OakLogBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = OakLogBlock::RENDER;

pub struct BlockOakLogMod;

impl BlockOakLogMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
