use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PolishedCinnabarBlock;

impl Block for PolishedCinnabarBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:polished-cinnabar",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PolishedCinnabarBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-polished-cinnabar:block/polished_cinnabar"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PolishedCinnabarBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PolishedCinnabarBlock::RENDER;

pub struct BlockPolishedCinnabarMod;

impl BlockPolishedCinnabarMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
