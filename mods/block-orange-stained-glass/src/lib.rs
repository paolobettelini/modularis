use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct OrangeStainedGlassBlock;

impl Block for OrangeStainedGlassBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:orange-stained-glass",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for OrangeStainedGlassBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-orange-stained-glass:block/orange_stained_glass"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = OrangeStainedGlassBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = OrangeStainedGlassBlock::RENDER;

pub struct BlockOrangeStainedGlassMod;

impl BlockOrangeStainedGlassMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
