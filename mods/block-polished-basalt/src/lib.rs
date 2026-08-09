use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PolishedBasaltBlock;

impl Block for PolishedBasaltBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:polished-basalt",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PolishedBasaltBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-polished-basalt:block/polished_basalt"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PolishedBasaltBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PolishedBasaltBlock::RENDER;

pub struct BlockPolishedBasaltMod;

impl BlockPolishedBasaltMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
