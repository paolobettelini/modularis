use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct RedConcreteBlock;

impl Block for RedConcreteBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:red-concrete",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for RedConcreteBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-red-concrete:block/red_concrete"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = RedConcreteBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = RedConcreteBlock::RENDER;

pub struct BlockRedConcreteMod;

impl BlockRedConcreteMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
