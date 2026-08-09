use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct TintedGlassBlock;

impl Block for TintedGlassBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:tinted-glass",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for TintedGlassBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-tinted-glass:block/tinted_glass"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = TintedGlassBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = TintedGlassBlock::RENDER;

pub struct BlockTintedGlassMod;

impl BlockTintedGlassMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
