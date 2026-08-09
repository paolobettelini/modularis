use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct LightGrayStainedGlassBlock;

impl Block for LightGrayStainedGlassBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:light-gray-stained-glass",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for LightGrayStainedGlassBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-light-gray-stained-glass:block/light_gray_stained_glass"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = LightGrayStainedGlassBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = LightGrayStainedGlassBlock::RENDER;

pub struct BlockLightGrayStainedGlassMod;

impl BlockLightGrayStainedGlassMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
