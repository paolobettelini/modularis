use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct GreenTerracottaBlock;

impl Block for GreenTerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:green-terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for GreenTerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-green-terracotta:block/green_terracotta"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = GreenTerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = GreenTerracottaBlock::RENDER;

pub struct BlockGreenTerracottaMod;

impl BlockGreenTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
