use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct ExposedCutCopperBlock;

impl Block for ExposedCutCopperBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:exposed-cut-copper",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for ExposedCutCopperBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-exposed-cut-copper:block/exposed_cut_copper"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = ExposedCutCopperBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = ExposedCutCopperBlock::RENDER;

pub struct BlockExposedCutCopperMod;

impl BlockExposedCutCopperMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
