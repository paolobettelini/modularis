use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PaleOakLeavesBlock;

impl Block for PaleOakLeavesBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:pale-oak-leaves",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PaleOakLeavesBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-pale-oak-leaves:block/pale_oak_leaves"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PaleOakLeavesBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PaleOakLeavesBlock::RENDER;

pub struct BlockPaleOakLeavesMod;

impl BlockPaleOakLeavesMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
