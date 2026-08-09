use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct StrippedWarpedStemBlock;

impl Block for StrippedWarpedStemBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:stripped-warped-stem",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for StrippedWarpedStemBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-stripped-warped-stem:block/stripped_warped_stem"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = StrippedWarpedStemBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = StrippedWarpedStemBlock::RENDER;

pub struct BlockStrippedWarpedStemMod;

impl BlockStrippedWarpedStemMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
