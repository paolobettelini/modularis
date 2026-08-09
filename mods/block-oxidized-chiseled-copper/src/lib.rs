use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct OxidizedChiseledCopperBlock;

impl Block for OxidizedChiseledCopperBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:oxidized-chiseled-copper",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for OxidizedChiseledCopperBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-oxidized-chiseled-copper:block/oxidized_chiseled_copper"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = OxidizedChiseledCopperBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = OxidizedChiseledCopperBlock::RENDER;

pub struct BlockOxidizedChiseledCopperMod;

impl BlockOxidizedChiseledCopperMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
