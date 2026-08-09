use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct BrownStainedGlassBlock;

impl Block for BrownStainedGlassBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:brown-stained-glass",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for BrownStainedGlassBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-brown-stained-glass:block/brown_stained_glass"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = BrownStainedGlassBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BrownStainedGlassBlock::RENDER;

pub struct BlockBrownStainedGlassMod;

impl BlockBrownStainedGlassMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
