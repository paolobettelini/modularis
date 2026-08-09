use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct LimeGlazedTerracottaBlock;

impl Block for LimeGlazedTerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:lime-glazed-terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for LimeGlazedTerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-lime-glazed-terracotta:block/lime_glazed_terracotta"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = LimeGlazedTerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = LimeGlazedTerracottaBlock::RENDER;

pub struct BlockLimeGlazedTerracottaMod;

impl BlockLimeGlazedTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
