use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct SmoothStoneBlock;

impl Block for SmoothStoneBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:smooth-stone",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for SmoothStoneBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-smooth-stone:block/smooth_stone"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = SmoothStoneBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = SmoothStoneBlock::RENDER;

pub struct BlockSmoothStoneMod;

impl BlockSmoothStoneMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
