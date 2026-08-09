use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PinkStainedGlassBlock;

impl Block for PinkStainedGlassBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:pink-stained-glass",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PinkStainedGlassBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-pink-stained-glass:block/pink_stained_glass"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PinkStainedGlassBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PinkStainedGlassBlock::RENDER;

pub struct BlockPinkStainedGlassMod;

impl BlockPinkStainedGlassMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
