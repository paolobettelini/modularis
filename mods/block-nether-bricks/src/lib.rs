use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct NetherBricksBlock;

impl Block for NetherBricksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:nether-bricks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for NetherBricksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-nether-bricks:block/nether_bricks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = NetherBricksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = NetherBricksBlock::RENDER;

pub struct BlockNetherBricksMod;

impl BlockNetherBricksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
