use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct BlueStainedGlassBlock;

impl Block for BlueStainedGlassBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:blue-stained-glass",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for BlueStainedGlassBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-blue-stained-glass:block/blue_stained_glass"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = BlueStainedGlassBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BlueStainedGlassBlock::RENDER;

pub struct BlockBlueStainedGlassMod;

impl BlockBlueStainedGlassMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
