use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct ExposedCopperGrateBlock;

impl Block for ExposedCopperGrateBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:exposed-copper-grate",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for ExposedCopperGrateBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-exposed-copper-grate:block/exposed_copper_grate"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = ExposedCopperGrateBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = ExposedCopperGrateBlock::RENDER;

pub struct BlockExposedCopperGrateMod;

impl BlockExposedCopperGrateMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
