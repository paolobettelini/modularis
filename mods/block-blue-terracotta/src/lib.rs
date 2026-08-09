use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct BlueTerracottaBlock;

impl Block for BlueTerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:blue-terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for BlueTerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-blue-terracotta:block/blue_terracotta"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = BlueTerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BlueTerracottaBlock::RENDER;

pub struct BlockBlueTerracottaMod;

impl BlockBlueTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
