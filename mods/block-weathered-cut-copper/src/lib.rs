use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct WeatheredCutCopperBlock;

impl Block for WeatheredCutCopperBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:weathered-cut-copper",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for WeatheredCutCopperBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-weathered-cut-copper:block/weathered_cut_copper"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = WeatheredCutCopperBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = WeatheredCutCopperBlock::RENDER;

pub struct BlockWeatheredCutCopperMod;

impl BlockWeatheredCutCopperMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
