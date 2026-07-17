use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct StoneBlock;

impl Block for StoneBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:stone",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for StoneBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-stone:block/stone"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = StoneBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = StoneBlock::RENDER;

pub struct BlockStoneMod;

impl BlockStoneMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
