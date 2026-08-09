use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct AzaleaLeavesBlock;

impl Block for AzaleaLeavesBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:azalea-leaves",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for AzaleaLeavesBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-azalea-leaves:block/azalea_leaves"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = AzaleaLeavesBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = AzaleaLeavesBlock::RENDER;

pub struct BlockAzaleaLeavesMod;

impl BlockAzaleaLeavesMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
