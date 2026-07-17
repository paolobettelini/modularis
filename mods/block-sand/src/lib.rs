use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct SandBlock;

impl Block for SandBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:sand",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for SandBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-sand:block/sand"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = SandBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = SandBlock::RENDER;

pub struct BlockSandMod;

impl BlockSandMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
