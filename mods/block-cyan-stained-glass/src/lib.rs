use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct CyanStainedGlassBlock;

impl Block for CyanStainedGlassBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:cyan-stained-glass",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for CyanStainedGlassBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-cyan-stained-glass:block/cyan_stained_glass"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = CyanStainedGlassBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CyanStainedGlassBlock::RENDER;

pub struct BlockCyanStainedGlassMod;

impl BlockCyanStainedGlassMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
