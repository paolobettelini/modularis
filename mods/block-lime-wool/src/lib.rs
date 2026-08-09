use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct LimeWoolBlock;

impl Block for LimeWoolBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:lime-wool",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for LimeWoolBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-lime-wool:block/lime_wool"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = LimeWoolBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = LimeWoolBlock::RENDER;

pub struct BlockLimeWoolMod;

impl BlockLimeWoolMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
