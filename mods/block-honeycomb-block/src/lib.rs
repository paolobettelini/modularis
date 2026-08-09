use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct HoneycombBlockBlock;

impl Block for HoneycombBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:honeycomb-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for HoneycombBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-honeycomb-block:block/honeycomb_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = HoneycombBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = HoneycombBlockBlock::RENDER;

pub struct BlockHoneycombBlockMod;

impl BlockHoneycombBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
