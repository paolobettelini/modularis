use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct CopperGrateBlock;

impl Block for CopperGrateBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:copper-grate",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for CopperGrateBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-copper-grate:block/copper_grate"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = CopperGrateBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CopperGrateBlock::RENDER;

pub struct BlockCopperGrateMod;

impl BlockCopperGrateMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
