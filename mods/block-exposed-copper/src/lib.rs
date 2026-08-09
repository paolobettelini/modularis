use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct ExposedCopperBlock;

impl Block for ExposedCopperBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:exposed-copper",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for ExposedCopperBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-exposed-copper:block/exposed_copper"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = ExposedCopperBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = ExposedCopperBlock::RENDER;

pub struct BlockExposedCopperMod;

impl BlockExposedCopperMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
