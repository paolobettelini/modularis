use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct RedTerracottaBlock;

impl Block for RedTerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:red-terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for RedTerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-red-terracotta:block/red_terracotta"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = RedTerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = RedTerracottaBlock::RENDER;

pub struct BlockRedTerracottaMod;

impl BlockRedTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
