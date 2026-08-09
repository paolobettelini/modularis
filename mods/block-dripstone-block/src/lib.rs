use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct DripstoneBlockBlock;

impl Block for DripstoneBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:dripstone-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for DripstoneBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-dripstone-block:block/dripstone_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = DripstoneBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = DripstoneBlockBlock::RENDER;

pub struct BlockDripstoneBlockMod;

impl BlockDripstoneBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
