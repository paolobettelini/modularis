use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct MagentaStainedGlassBlock;

impl Block for MagentaStainedGlassBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:magenta-stained-glass",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for MagentaStainedGlassBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-magenta-stained-glass:block/magenta_stained_glass"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = MagentaStainedGlassBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = MagentaStainedGlassBlock::RENDER;

pub struct BlockMagentaStainedGlassMod;

impl BlockMagentaStainedGlassMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
