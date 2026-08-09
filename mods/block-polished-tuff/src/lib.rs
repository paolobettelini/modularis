use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PolishedTuffBlock;

impl Block for PolishedTuffBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:polished-tuff",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PolishedTuffBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-polished-tuff:block/polished_tuff"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PolishedTuffBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PolishedTuffBlock::RENDER;

pub struct BlockPolishedTuffMod;

impl BlockPolishedTuffMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
