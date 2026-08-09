use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct OxidizedCopperGrateBlock;

impl Block for OxidizedCopperGrateBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:oxidized-copper-grate",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for OxidizedCopperGrateBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-oxidized-copper-grate:block/oxidized_copper_grate"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = OxidizedCopperGrateBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = OxidizedCopperGrateBlock::RENDER;

pub struct BlockOxidizedCopperGrateMod;

impl BlockOxidizedCopperGrateMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
