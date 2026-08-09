use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct TuffBricksBlock;

impl Block for TuffBricksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:tuff-bricks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for TuffBricksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-tuff-bricks:block/tuff_bricks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = TuffBricksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = TuffBricksBlock::RENDER;

pub struct BlockTuffBricksMod;

impl BlockTuffBricksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
