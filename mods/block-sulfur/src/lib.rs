use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct SulfurBlock;

impl Block for SulfurBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:sulfur",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for SulfurBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-sulfur:block/sulfur"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = SulfurBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = SulfurBlock::RENDER;

pub struct BlockSulfurMod;

impl BlockSulfurMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
