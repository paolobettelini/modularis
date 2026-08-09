use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct GlassBlock;

impl Block for GlassBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:glass",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for GlassBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-glass:block/glass"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = GlassBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = GlassBlock::RENDER;

pub struct BlockGlassMod;

impl BlockGlassMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
