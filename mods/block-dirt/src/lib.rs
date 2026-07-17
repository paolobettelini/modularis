use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct DirtBlock;

impl Block for DirtBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:dirt",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for DirtBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-dirt:block/dirt"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = DirtBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = DirtBlock::RENDER;

pub struct BlockDirtMod;

impl BlockDirtMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
