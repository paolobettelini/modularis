use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct GreenStainedGlassBlock;

impl Block for GreenStainedGlassBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:green-stained-glass",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for GreenStainedGlassBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-green-stained-glass:block/green_stained_glass"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = GreenStainedGlassBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = GreenStainedGlassBlock::RENDER;

pub struct BlockGreenStainedGlassMod;

impl BlockGreenStainedGlassMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
