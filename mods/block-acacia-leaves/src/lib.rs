use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct AcaciaLeavesBlock;

impl Block for AcaciaLeavesBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:acacia-leaves",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for AcaciaLeavesBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-acacia-leaves:block/acacia_leaves"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = AcaciaLeavesBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = AcaciaLeavesBlock::RENDER;

pub struct BlockAcaciaLeavesMod;

impl BlockAcaciaLeavesMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
