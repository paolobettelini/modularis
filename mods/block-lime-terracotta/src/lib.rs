use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct LimeTerracottaBlock;

impl Block for LimeTerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:lime-terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for LimeTerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-lime-terracotta:block/lime_terracotta"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = LimeTerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = LimeTerracottaBlock::RENDER;

pub struct BlockLimeTerracottaMod;

impl BlockLimeTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
