use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct TuffBlock;

impl Block for TuffBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:tuff",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for TuffBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-tuff:block/tuff"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = TuffBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = TuffBlock::RENDER;

pub struct BlockTuffMod;

impl BlockTuffMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
