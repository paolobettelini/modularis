use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct ChiseledDeepslateBlock;

impl Block for ChiseledDeepslateBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:chiseled-deepslate",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for ChiseledDeepslateBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-chiseled-deepslate:block/chiseled_deepslate"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = ChiseledDeepslateBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = ChiseledDeepslateBlock::RENDER;

pub struct BlockChiseledDeepslateMod;

impl BlockChiseledDeepslateMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
