use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct CyanConcreteBlock;

impl Block for CyanConcreteBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:cyan-concrete",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for CyanConcreteBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-cyan-concrete:block/cyan_concrete"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = CyanConcreteBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CyanConcreteBlock::RENDER;

pub struct BlockCyanConcreteMod;

impl BlockCyanConcreteMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
