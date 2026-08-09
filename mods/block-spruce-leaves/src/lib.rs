use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct SpruceLeavesBlock;

impl Block for SpruceLeavesBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:spruce-leaves",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for SpruceLeavesBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-spruce-leaves:block/spruce_leaves"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = SpruceLeavesBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = SpruceLeavesBlock::RENDER;

pub struct BlockSpruceLeavesMod;

impl BlockSpruceLeavesMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
