use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct ChiseledTuffBlock;

impl Block for ChiseledTuffBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:chiseled-tuff",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for ChiseledTuffBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-chiseled-tuff:block/chiseled_tuff"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = ChiseledTuffBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = ChiseledTuffBlock::RENDER;

pub struct BlockChiseledTuffMod;

impl BlockChiseledTuffMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
