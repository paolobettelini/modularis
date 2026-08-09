use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct StrippedCrimsonStemBlock;

impl Block for StrippedCrimsonStemBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:stripped-crimson-stem",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for StrippedCrimsonStemBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-stripped-crimson-stem:block/stripped_crimson_stem"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = StrippedCrimsonStemBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = StrippedCrimsonStemBlock::RENDER;

pub struct BlockStrippedCrimsonStemMod;

impl BlockStrippedCrimsonStemMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
