use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct BambooPlanksBlock;

impl Block for BambooPlanksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:bamboo-planks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for BambooPlanksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-bamboo-planks:block/bamboo_planks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = BambooPlanksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BambooPlanksBlock::RENDER;

pub struct BlockBambooPlanksMod;

impl BlockBambooPlanksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
