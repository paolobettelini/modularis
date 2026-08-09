use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct ResinBricksBlock;

impl Block for ResinBricksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:resin-bricks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for ResinBricksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-resin-bricks:block/resin_bricks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = ResinBricksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = ResinBricksBlock::RENDER;

pub struct BlockResinBricksMod;

impl BlockResinBricksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
