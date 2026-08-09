use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PurpleGlazedTerracottaBlock;

impl Block for PurpleGlazedTerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:purple-glazed-terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PurpleGlazedTerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-purple-glazed-terracotta:block/purple_glazed_terracotta"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PurpleGlazedTerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PurpleGlazedTerracottaBlock::RENDER;

pub struct BlockPurpleGlazedTerracottaMod;

impl BlockPurpleGlazedTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
