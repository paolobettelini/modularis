use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct ShroomlightBlock;

impl Block for ShroomlightBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:shroomlight",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for ShroomlightBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-shroomlight:block/shroomlight"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = ShroomlightBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = ShroomlightBlock::RENDER;

pub struct BlockShroomlightMod;

impl BlockShroomlightMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
