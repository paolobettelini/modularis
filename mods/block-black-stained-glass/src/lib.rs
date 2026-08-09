use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct BlackStainedGlassBlock;

impl Block for BlackStainedGlassBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:black-stained-glass",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for BlackStainedGlassBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-black-stained-glass:block/black_stained_glass"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = BlackStainedGlassBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BlackStainedGlassBlock::RENDER;

pub struct BlockBlackStainedGlassMod;

impl BlockBlackStainedGlassMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
