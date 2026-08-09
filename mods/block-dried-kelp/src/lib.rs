use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct DriedKelpBlock;

impl Block for DriedKelpBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:dried-kelp",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for DriedKelpBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-dried-kelp:block/dried_kelp"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = DriedKelpBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = DriedKelpBlock::RENDER;

pub struct BlockDriedKelpMod;

impl BlockDriedKelpMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
