use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct MangroveLeavesBlock;

impl Block for MangroveLeavesBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:mangrove-leaves",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for MangroveLeavesBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-mangrove-leaves:block/mangrove_leaves"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = MangroveLeavesBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = MangroveLeavesBlock::RENDER;

pub struct BlockMangroveLeavesMod;

impl BlockMangroveLeavesMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
