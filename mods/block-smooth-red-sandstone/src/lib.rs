use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct SmoothRedSandstoneBlock;

impl Block for SmoothRedSandstoneBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:smooth-red-sandstone",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for SmoothRedSandstoneBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-smooth-red-sandstone:block/smooth_red_sandstone"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = SmoothRedSandstoneBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = SmoothRedSandstoneBlock::RENDER;

pub struct BlockSmoothRedSandstoneMod;

impl BlockSmoothRedSandstoneMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
