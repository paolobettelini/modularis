use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct BambooBlockBlock;

impl Block for BambooBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:bamboo-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for BambooBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-bamboo-block:block/bamboo_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = BambooBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BambooBlockBlock::RENDER;

pub struct BlockBambooBlockMod;

impl BlockBambooBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
