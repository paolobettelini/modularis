use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct GrayStainedGlassBlock;

impl Block for GrayStainedGlassBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:gray-stained-glass",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for GrayStainedGlassBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-gray-stained-glass:block/gray_stained_glass"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = GrayStainedGlassBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = GrayStainedGlassBlock::RENDER;

pub struct BlockGrayStainedGlassMod;

impl BlockGrayStainedGlassMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
