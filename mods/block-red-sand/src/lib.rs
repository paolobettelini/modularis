use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct RedSandBlock;

impl Block for RedSandBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:red-sand",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for RedSandBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-red-sand:block/red_sand"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = RedSandBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = RedSandBlock::RENDER;

pub struct BlockRedSandMod;
impl BlockRedSandMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
