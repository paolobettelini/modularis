use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct RedGlazedTerracottaBlock;

impl Block for RedGlazedTerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:red-glazed-terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for RedGlazedTerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-red-glazed-terracotta:block/red_glazed_terracotta"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = RedGlazedTerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = RedGlazedTerracottaBlock::RENDER;

pub struct BlockRedGlazedTerracottaMod;

impl BlockRedGlazedTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
