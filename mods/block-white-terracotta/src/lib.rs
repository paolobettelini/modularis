use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct WhiteTerracottaBlock;

impl Block for WhiteTerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:white-terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for WhiteTerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-white-terracotta:block/white_terracotta"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = WhiteTerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = WhiteTerracottaBlock::RENDER;

pub struct BlockWhiteTerracottaMod;

impl BlockWhiteTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
