use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct LightBlueStainedGlassBlock;

impl Block for LightBlueStainedGlassBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:light-blue-stained-glass",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for LightBlueStainedGlassBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-light-blue-stained-glass:block/light_blue_stained_glass"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = LightBlueStainedGlassBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = LightBlueStainedGlassBlock::RENDER;

pub struct BlockLightBlueStainedGlassMod;

impl BlockLightBlueStainedGlassMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
