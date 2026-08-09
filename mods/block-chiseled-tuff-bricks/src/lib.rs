use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct ChiseledTuffBricksBlock;

impl Block for ChiseledTuffBricksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:chiseled-tuff-bricks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for ChiseledTuffBricksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-chiseled-tuff-bricks:block/chiseled_tuff_bricks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = ChiseledTuffBricksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = ChiseledTuffBricksBlock::RENDER;

pub struct BlockChiseledTuffBricksMod;

impl BlockChiseledTuffBricksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
