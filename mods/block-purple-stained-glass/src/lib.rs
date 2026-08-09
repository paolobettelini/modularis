use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PurpleStainedGlassBlock;

impl Block for PurpleStainedGlassBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:purple-stained-glass",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PurpleStainedGlassBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-purple-stained-glass:block/purple_stained_glass"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PurpleStainedGlassBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PurpleStainedGlassBlock::RENDER;

pub struct BlockPurpleStainedGlassMod;

impl BlockPurpleStainedGlassMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
