use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct CyanGlazedTerracottaBlock;

impl Block for CyanGlazedTerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:cyan-glazed-terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for CyanGlazedTerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-cyan-glazed-terracotta:block/cyan_glazed_terracotta"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = CyanGlazedTerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CyanGlazedTerracottaBlock::RENDER;

pub struct BlockCyanGlazedTerracottaMod;

impl BlockCyanGlazedTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
