use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct BlackTerracottaBlock;

impl Block for BlackTerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:black-terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for BlackTerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-black-terracotta:block/black_terracotta"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = BlackTerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BlackTerracottaBlock::RENDER;

pub struct BlockBlackTerracottaMod;

impl BlockBlackTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
