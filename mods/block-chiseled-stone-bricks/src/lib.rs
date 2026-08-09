use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct ChiseledStoneBricksBlock;

impl Block for ChiseledStoneBricksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:chiseled-stone-bricks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for ChiseledStoneBricksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-chiseled-stone-bricks:block/chiseled_stone_bricks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = ChiseledStoneBricksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = ChiseledStoneBricksBlock::RENDER;

pub struct BlockChiseledStoneBricksMod;

impl BlockChiseledStoneBricksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
