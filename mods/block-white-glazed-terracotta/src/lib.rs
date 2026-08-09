use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct WhiteGlazedTerracottaBlock;

impl Block for WhiteGlazedTerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:white-glazed-terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for WhiteGlazedTerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-white-glazed-terracotta:block/white_glazed_terracotta"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = WhiteGlazedTerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = WhiteGlazedTerracottaBlock::RENDER;

pub struct BlockWhiteGlazedTerracottaMod;

impl BlockWhiteGlazedTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
