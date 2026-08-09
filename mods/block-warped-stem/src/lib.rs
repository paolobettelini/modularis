use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct WarpedStemBlock;

impl Block for WarpedStemBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:warped-stem",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for WarpedStemBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-warped-stem:block/warped_stem"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = WarpedStemBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = WarpedStemBlock::RENDER;

pub struct BlockWarpedStemMod;

impl BlockWarpedStemMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
