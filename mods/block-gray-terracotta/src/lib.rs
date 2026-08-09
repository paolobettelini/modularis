use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct GrayTerracottaBlock;

impl Block for GrayTerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:gray-terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for GrayTerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-gray-terracotta:block/gray_terracotta"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = GrayTerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = GrayTerracottaBlock::RENDER;

pub struct BlockGrayTerracottaMod;

impl BlockGrayTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
