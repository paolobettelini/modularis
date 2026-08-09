use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct GreenGlazedTerracottaBlock;

impl Block for GreenGlazedTerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:green-glazed-terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for GreenGlazedTerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-green-glazed-terracotta:block/green_glazed_terracotta"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = GreenGlazedTerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = GreenGlazedTerracottaBlock::RENDER;

pub struct BlockGreenGlazedTerracottaMod;

impl BlockGreenGlazedTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
