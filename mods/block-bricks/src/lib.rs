use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct BricksBlock;

impl Block for BricksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:bricks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for BricksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-bricks:block/bricks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = BricksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BricksBlock::RENDER;

pub struct BlockBricksMod;

impl BlockBricksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
