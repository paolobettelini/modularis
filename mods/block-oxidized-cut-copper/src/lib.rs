use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct OxidizedCutCopperBlock;

impl Block for OxidizedCutCopperBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:oxidized-cut-copper",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for OxidizedCutCopperBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-oxidized-cut-copper:block/oxidized_cut_copper"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = OxidizedCutCopperBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = OxidizedCutCopperBlock::RENDER;

pub struct BlockOxidizedCutCopperMod;

impl BlockOxidizedCutCopperMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
