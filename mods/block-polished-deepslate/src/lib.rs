use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PolishedDeepslateBlock;

impl Block for PolishedDeepslateBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:polished-deepslate",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PolishedDeepslateBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-polished-deepslate:block/polished_deepslate"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PolishedDeepslateBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PolishedDeepslateBlock::RENDER;

pub struct BlockPolishedDeepslateMod;

impl BlockPolishedDeepslateMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
