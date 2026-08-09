use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PinkGlazedTerracottaBlock;

impl Block for PinkGlazedTerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:pink-glazed-terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PinkGlazedTerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-pink-glazed-terracotta:block/pink_glazed_terracotta"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PinkGlazedTerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PinkGlazedTerracottaBlock::RENDER;

pub struct BlockPinkGlazedTerracottaMod;

impl BlockPinkGlazedTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
