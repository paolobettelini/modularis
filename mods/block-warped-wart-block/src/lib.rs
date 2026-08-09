use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct WarpedWartBlockBlock;

impl Block for WarpedWartBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:warped-wart-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for WarpedWartBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-warped-wart-block:block/warped_wart_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = WarpedWartBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = WarpedWartBlockBlock::RENDER;

pub struct BlockWarpedWartBlockMod;

impl BlockWarpedWartBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
