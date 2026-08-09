use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct LightGrayGlazedTerracottaBlock;

impl Block for LightGrayGlazedTerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:light-gray-glazed-terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for LightGrayGlazedTerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-light-gray-glazed-terracotta:block/light_gray_glazed_terracotta"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = LightGrayGlazedTerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = LightGrayGlazedTerracottaBlock::RENDER;

pub struct BlockLightGrayGlazedTerracottaMod;

impl BlockLightGrayGlazedTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
