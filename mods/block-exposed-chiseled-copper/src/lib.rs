use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct ExposedChiseledCopperBlock;

impl Block for ExposedChiseledCopperBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:exposed-chiseled-copper",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for ExposedChiseledCopperBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-exposed-chiseled-copper:block/exposed_chiseled_copper"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = ExposedChiseledCopperBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = ExposedChiseledCopperBlock::RENDER;

pub struct BlockExposedChiseledCopperMod;

impl BlockExposedChiseledCopperMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
