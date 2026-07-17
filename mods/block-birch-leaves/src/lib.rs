use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct BirchLeavesBlock;

impl Block for BirchLeavesBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:birch-leaves",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for BirchLeavesBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-birch-leaves:block/birch_leaves"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = BirchLeavesBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BirchLeavesBlock::RENDER;

pub struct BlockBirchLeavesMod;

impl BlockBirchLeavesMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
