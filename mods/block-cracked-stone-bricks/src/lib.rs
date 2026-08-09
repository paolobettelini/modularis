use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct CrackedStoneBricksBlock;

impl Block for CrackedStoneBricksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:cracked-stone-bricks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for CrackedStoneBricksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-cracked-stone-bricks:block/cracked_stone_bricks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = CrackedStoneBricksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CrackedStoneBricksBlock::RENDER;

pub struct BlockCrackedStoneBricksMod;

impl BlockCrackedStoneBricksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
