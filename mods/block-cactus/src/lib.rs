use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct CactusBlock;

impl Block for CactusBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:cactus",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for CactusBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-cactus:block/cactus"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = CactusBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CactusBlock::RENDER;

pub struct BlockCactusMod;

impl BlockCactusMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
