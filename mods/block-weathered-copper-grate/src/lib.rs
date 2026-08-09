use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct WeatheredCopperGrateBlock;

impl Block for WeatheredCopperGrateBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:weathered-copper-grate",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for WeatheredCopperGrateBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-weathered-copper-grate:block/weathered_copper_grate"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = WeatheredCopperGrateBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = WeatheredCopperGrateBlock::RENDER;

pub struct BlockWeatheredCopperGrateMod;

impl BlockWeatheredCopperGrateMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
