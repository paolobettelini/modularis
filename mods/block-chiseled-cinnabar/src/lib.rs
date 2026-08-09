use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct ChiseledCinnabarBlock;

impl Block for ChiseledCinnabarBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:chiseled-cinnabar",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for ChiseledCinnabarBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-chiseled-cinnabar:block/chiseled_cinnabar"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = ChiseledCinnabarBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = ChiseledCinnabarBlock::RENDER;

pub struct BlockChiseledCinnabarMod;

impl BlockChiseledCinnabarMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
