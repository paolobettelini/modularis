use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct FloweringAzaleaLeavesBlock;

impl Block for FloweringAzaleaLeavesBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:flowering-azalea-leaves",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for FloweringAzaleaLeavesBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-flowering-azalea-leaves:block/flowering_azalea_leaves"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = FloweringAzaleaLeavesBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = FloweringAzaleaLeavesBlock::RENDER;

pub struct BlockFloweringAzaleaLeavesMod;

impl BlockFloweringAzaleaLeavesMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
