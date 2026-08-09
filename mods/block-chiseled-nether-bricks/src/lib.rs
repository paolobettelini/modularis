use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct ChiseledNetherBricksBlock;

impl Block for ChiseledNetherBricksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:chiseled-nether-bricks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for ChiseledNetherBricksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-chiseled-nether-bricks:block/chiseled_nether_bricks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = ChiseledNetherBricksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = ChiseledNetherBricksBlock::RENDER;

pub struct BlockChiseledNetherBricksMod;

impl BlockChiseledNetherBricksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
