use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct CrimsonStemBlock;

impl Block for CrimsonStemBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:crimson-stem",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for CrimsonStemBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-crimson-stem:block/crimson_stem"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = CrimsonStemBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CrimsonStemBlock::RENDER;

pub struct BlockCrimsonStemMod;

impl BlockCrimsonStemMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
