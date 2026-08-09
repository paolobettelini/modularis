use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct DarkOakLeavesBlock;

impl Block for DarkOakLeavesBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:dark-oak-leaves",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for DarkOakLeavesBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-dark-oak-leaves:block/dark_oak_leaves"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = DarkOakLeavesBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = DarkOakLeavesBlock::RENDER;

pub struct BlockDarkOakLeavesMod;

impl BlockDarkOakLeavesMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
