use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct OxidizedCopperBlock;

impl Block for OxidizedCopperBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:oxidized-copper",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for OxidizedCopperBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-oxidized-copper:block/oxidized_copper"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = OxidizedCopperBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = OxidizedCopperBlock::RENDER;

pub struct BlockOxidizedCopperMod;

impl BlockOxidizedCopperMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
