use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct ChiseledResinBricksBlock;

impl Block for ChiseledResinBricksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:chiseled-resin-bricks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for ChiseledResinBricksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-chiseled-resin-bricks:block/chiseled_resin_bricks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = ChiseledResinBricksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = ChiseledResinBricksBlock::RENDER;

pub struct BlockChiseledResinBricksMod;

impl BlockChiseledResinBricksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
