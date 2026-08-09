use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct CherryLeavesBlock;

impl Block for CherryLeavesBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:cherry-leaves",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for CherryLeavesBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-cherry-leaves:block/cherry_leaves"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = CherryLeavesBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CherryLeavesBlock::RENDER;

pub struct BlockCherryLeavesMod;

impl BlockCherryLeavesMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
