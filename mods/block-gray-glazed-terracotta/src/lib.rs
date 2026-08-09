use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct GrayGlazedTerracottaBlock;

impl Block for GrayGlazedTerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:gray-glazed-terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for GrayGlazedTerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-gray-glazed-terracotta:block/gray_glazed_terracotta"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = GrayGlazedTerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = GrayGlazedTerracottaBlock::RENDER;

pub struct BlockGrayGlazedTerracottaMod;

impl BlockGrayGlazedTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
