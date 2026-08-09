use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct ClayBlock;

impl Block for ClayBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:clay",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for ClayBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-clay:block/clay"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = ClayBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = ClayBlock::RENDER;

pub struct BlockClayMod;

impl BlockClayMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
