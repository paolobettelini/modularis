use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct StoneBricksBlock;

impl Block for StoneBricksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:stone-bricks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for StoneBricksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-stone-bricks:block/stone_bricks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = StoneBricksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = StoneBricksBlock::RENDER;

pub struct BlockStoneBricksMod;

impl BlockStoneBricksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
