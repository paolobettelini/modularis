use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PinkTerracottaBlock;

impl Block for PinkTerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:pink-terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PinkTerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-pink-terracotta:block/pink_terracotta"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PinkTerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PinkTerracottaBlock::RENDER;

pub struct BlockPinkTerracottaMod;

impl BlockPinkTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
