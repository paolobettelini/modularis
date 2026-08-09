use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PurpleTerracottaBlock;

impl Block for PurpleTerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:purple-terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PurpleTerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-purple-terracotta:block/purple_terracotta"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PurpleTerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PurpleTerracottaBlock::RENDER;

pub struct BlockPurpleTerracottaMod;

impl BlockPurpleTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
