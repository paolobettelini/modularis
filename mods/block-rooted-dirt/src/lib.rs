use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct RootedDirtBlock;

impl Block for RootedDirtBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:rooted-dirt",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for RootedDirtBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-rooted-dirt:block/rooted_dirt"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = RootedDirtBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = RootedDirtBlock::RENDER;

pub struct BlockRootedDirtMod;

impl BlockRootedDirtMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
