use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct RedStainedGlassBlock;

impl Block for RedStainedGlassBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:red-stained-glass",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for RedStainedGlassBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-red-stained-glass:block/red_stained_glass"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = RedStainedGlassBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = RedStainedGlassBlock::RENDER;

pub struct BlockRedStainedGlassMod;

impl BlockRedStainedGlassMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
