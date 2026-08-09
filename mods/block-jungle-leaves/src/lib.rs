use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct JungleLeavesBlock;

impl Block for JungleLeavesBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:jungle-leaves",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for JungleLeavesBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-jungle-leaves:block/jungle_leaves"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = JungleLeavesBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = JungleLeavesBlock::RENDER;

pub struct BlockJungleLeavesMod;

impl BlockJungleLeavesMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
