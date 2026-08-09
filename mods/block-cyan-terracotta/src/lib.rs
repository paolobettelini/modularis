use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct CyanTerracottaBlock;

impl Block for CyanTerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:cyan-terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for CyanTerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-cyan-terracotta:block/cyan_terracotta"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = CyanTerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CyanTerracottaBlock::RENDER;

pub struct BlockCyanTerracottaMod;

impl BlockCyanTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
