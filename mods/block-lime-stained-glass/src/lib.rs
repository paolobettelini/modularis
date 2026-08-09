use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct LimeStainedGlassBlock;

impl Block for LimeStainedGlassBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:lime-stained-glass",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for LimeStainedGlassBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-lime-stained-glass:block/lime_stained_glass"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = LimeStainedGlassBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = LimeStainedGlassBlock::RENDER;

pub struct BlockLimeStainedGlassMod;

impl BlockLimeStainedGlassMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
