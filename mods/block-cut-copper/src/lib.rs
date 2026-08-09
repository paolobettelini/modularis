use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct CutCopperBlock;

impl Block for CutCopperBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:cut-copper",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for CutCopperBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-cut-copper:block/cut_copper"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = CutCopperBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CutCopperBlock::RENDER;

pub struct BlockCutCopperMod;

impl BlockCutCopperMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
