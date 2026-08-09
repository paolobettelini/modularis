use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct ChiseledSulfurBlock;

impl Block for ChiseledSulfurBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:chiseled-sulfur",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for ChiseledSulfurBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-chiseled-sulfur:block/chiseled_sulfur"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = ChiseledSulfurBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = ChiseledSulfurBlock::RENDER;

pub struct BlockChiseledSulfurMod;

impl BlockChiseledSulfurMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
