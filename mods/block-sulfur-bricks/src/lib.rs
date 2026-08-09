use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct SulfurBricksBlock;

impl Block for SulfurBricksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:sulfur-bricks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for SulfurBricksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-sulfur-bricks:block/sulfur_bricks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = SulfurBricksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = SulfurBricksBlock::RENDER;

pub struct BlockSulfurBricksMod;

impl BlockSulfurBricksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
