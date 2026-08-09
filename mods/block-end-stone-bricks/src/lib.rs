use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct EndStoneBricksBlock;

impl Block for EndStoneBricksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:end-stone-bricks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for EndStoneBricksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-end-stone-bricks:block/end_stone_bricks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = EndStoneBricksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = EndStoneBricksBlock::RENDER;

pub struct BlockEndStoneBricksMod;

impl BlockEndStoneBricksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
