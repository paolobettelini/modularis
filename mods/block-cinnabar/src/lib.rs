use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct CinnabarBlock;

impl Block for CinnabarBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:cinnabar",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for CinnabarBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-cinnabar:block/cinnabar"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = CinnabarBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CinnabarBlock::RENDER;

pub struct BlockCinnabarMod;

impl BlockCinnabarMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
