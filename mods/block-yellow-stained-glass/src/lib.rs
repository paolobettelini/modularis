use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct YellowStainedGlassBlock;

impl Block for YellowStainedGlassBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:yellow-stained-glass",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for YellowStainedGlassBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-yellow-stained-glass:block/yellow_stained_glass"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = YellowStainedGlassBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = YellowStainedGlassBlock::RENDER;

pub struct BlockYellowStainedGlassMod;

impl BlockYellowStainedGlassMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
