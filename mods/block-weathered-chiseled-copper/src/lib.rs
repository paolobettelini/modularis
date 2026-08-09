use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct WeatheredChiseledCopperBlock;

impl Block for WeatheredChiseledCopperBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:weathered-chiseled-copper",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for WeatheredChiseledCopperBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-weathered-chiseled-copper:block/weathered_chiseled_copper"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = WeatheredChiseledCopperBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = WeatheredChiseledCopperBlock::RENDER;

pub struct BlockWeatheredChiseledCopperMod;

impl BlockWeatheredChiseledCopperMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
