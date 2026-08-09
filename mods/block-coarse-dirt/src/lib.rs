use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct CoarseDirtBlock;

impl Block for CoarseDirtBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:coarse-dirt",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for CoarseDirtBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-coarse-dirt:block/coarse_dirt"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = CoarseDirtBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CoarseDirtBlock::RENDER;

pub struct BlockCoarseDirtMod;

impl BlockCoarseDirtMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
