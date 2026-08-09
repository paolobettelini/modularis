use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct WeatheredCopperBlock;

impl Block for WeatheredCopperBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:weathered-copper",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for WeatheredCopperBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-weathered-copper:block/weathered_copper"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = WeatheredCopperBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = WeatheredCopperBlock::RENDER;

pub struct BlockWeatheredCopperMod;

impl BlockWeatheredCopperMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
