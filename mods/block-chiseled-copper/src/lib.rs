use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct ChiseledCopperBlock;

impl Block for ChiseledCopperBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:chiseled-copper",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for ChiseledCopperBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-chiseled-copper:block/chiseled_copper"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = ChiseledCopperBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = ChiseledCopperBlock::RENDER;

pub struct BlockChiseledCopperMod;

impl BlockChiseledCopperMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
