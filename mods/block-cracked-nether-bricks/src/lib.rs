use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct CrackedNetherBricksBlock;

impl Block for CrackedNetherBricksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:cracked-nether-bricks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for CrackedNetherBricksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-cracked-nether-bricks:block/cracked_nether_bricks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = CrackedNetherBricksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CrackedNetherBricksBlock::RENDER;

pub struct BlockCrackedNetherBricksMod;

impl BlockCrackedNetherBricksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
