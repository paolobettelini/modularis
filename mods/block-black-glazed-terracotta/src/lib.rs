use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct BlackGlazedTerracottaBlock;

impl Block for BlackGlazedTerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:black-glazed-terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for BlackGlazedTerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-black-glazed-terracotta:block/black_glazed_terracotta"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = BlackGlazedTerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BlackGlazedTerracottaBlock::RENDER;

pub struct BlockBlackGlazedTerracottaMod;

impl BlockBlackGlazedTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
