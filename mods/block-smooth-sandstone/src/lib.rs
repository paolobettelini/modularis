use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct SmoothSandstoneBlock;

impl Block for SmoothSandstoneBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:smooth-sandstone",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for SmoothSandstoneBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-smooth-sandstone:block/smooth_sandstone"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = SmoothSandstoneBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = SmoothSandstoneBlock::RENDER;

pub struct BlockSmoothSandstoneMod;

impl BlockSmoothSandstoneMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
