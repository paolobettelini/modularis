use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct CobbledDeepslateBlock;

impl Block for CobbledDeepslateBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:cobbled-deepslate",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for CobbledDeepslateBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-cobbled-deepslate:block/cobbled_deepslate"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = CobbledDeepslateBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CobbledDeepslateBlock::RENDER;

pub struct BlockCobbledDeepslateMod;

impl BlockCobbledDeepslateMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
