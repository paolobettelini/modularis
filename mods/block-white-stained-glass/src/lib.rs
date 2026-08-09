use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct WhiteStainedGlassBlock;

impl Block for WhiteStainedGlassBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:white-stained-glass",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for WhiteStainedGlassBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-white-stained-glass:block/white_stained_glass"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = WhiteStainedGlassBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = WhiteStainedGlassBlock::RENDER;

pub struct BlockWhiteStainedGlassMod;

impl BlockWhiteStainedGlassMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
