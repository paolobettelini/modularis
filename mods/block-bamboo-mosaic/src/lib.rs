use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct BambooMosaicBlock;

impl Block for BambooMosaicBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:bamboo-mosaic",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for BambooMosaicBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-bamboo-mosaic:block/bamboo_mosaic"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = BambooMosaicBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BambooMosaicBlock::RENDER;

pub struct BlockBambooMosaicMod;

impl BlockBambooMosaicMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
