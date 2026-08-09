use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct RedNetherBricksBlock;

impl Block for RedNetherBricksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:red-nether-bricks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for RedNetherBricksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-red-nether-bricks:block/red_nether_bricks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = RedNetherBricksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = RedNetherBricksBlock::RENDER;

pub struct BlockRedNetherBricksMod;

impl BlockRedNetherBricksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
