use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct LimeConcreteBlock;

impl Block for LimeConcreteBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:lime-concrete",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for LimeConcreteBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-lime-concrete:block/lime_concrete"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = LimeConcreteBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = LimeConcreteBlock::RENDER;

pub struct BlockLimeConcreteMod;

impl BlockLimeConcreteMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
