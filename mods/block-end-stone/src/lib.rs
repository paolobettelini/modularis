use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct EndStoneBlock;

impl Block for EndStoneBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:end-stone",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for EndStoneBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-end-stone:block/end_stone"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = EndStoneBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = EndStoneBlock::RENDER;

pub struct BlockEndStoneMod;

impl BlockEndStoneMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
