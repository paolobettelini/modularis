use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct SculkBlock;

impl Block for SculkBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:sculk",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for SculkBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-sculk:block/sculk"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = SculkBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = SculkBlock::RENDER;

pub struct BlockSculkMod;

impl BlockSculkMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
