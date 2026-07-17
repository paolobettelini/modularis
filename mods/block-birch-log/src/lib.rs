use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct BirchLogBlock;

impl Block for BirchLogBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:birch-log",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for BirchLogBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-birch-log:block/birch_log"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = BirchLogBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BirchLogBlock::RENDER;

pub struct BlockBirchLogMod;

impl BlockBirchLogMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
