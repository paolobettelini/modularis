use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct LightGrayTerracottaBlock;

impl Block for LightGrayTerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:light-gray-terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for LightGrayTerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-light-gray-terracotta:block/light_gray_terracotta"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = LightGrayTerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = LightGrayTerracottaBlock::RENDER;

pub struct BlockLightGrayTerracottaMod;

impl BlockLightGrayTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
