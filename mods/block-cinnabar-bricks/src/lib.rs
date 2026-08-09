use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct CinnabarBricksBlock;

impl Block for CinnabarBricksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:cinnabar-bricks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for CinnabarBricksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-cinnabar-bricks:block/cinnabar_bricks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = CinnabarBricksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CinnabarBricksBlock::RENDER;

pub struct BlockCinnabarBricksMod;

impl BlockCinnabarBricksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
