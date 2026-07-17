use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct GlowstoneBlock;

impl Block for GlowstoneBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:glowstone",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for GlowstoneBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-glowstone:block/glowstone"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = GlowstoneBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = GlowstoneBlock::RENDER;

pub struct BlockGlowstoneMod;

impl BlockGlowstoneMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
