use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct OakLeavesBlock;

impl Block for OakLeavesBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:oak-leaves",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for OakLeavesBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-oak-leaves:block/oak_leaves"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = OakLeavesBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = OakLeavesBlock::RENDER;

pub struct BlockOakLeavesMod;

impl BlockOakLeavesMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
