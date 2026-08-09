use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PaleOakLogBlock;

impl Block for PaleOakLogBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:pale-oak-log",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PaleOakLogBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-pale-oak-log:block/pale_oak_log"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PaleOakLogBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PaleOakLogBlock::RENDER;

pub struct BlockPaleOakLogMod;

impl BlockPaleOakLogMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
