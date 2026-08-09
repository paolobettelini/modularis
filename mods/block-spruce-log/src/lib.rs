use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct SpruceLogBlock;

impl Block for SpruceLogBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:spruce-log",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for SpruceLogBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-spruce-log:block/spruce_log"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = SpruceLogBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = SpruceLogBlock::RENDER;

pub struct BlockSpruceLogMod;

impl BlockSpruceLogMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
