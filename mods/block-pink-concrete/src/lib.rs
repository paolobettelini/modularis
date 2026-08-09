use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PinkConcreteBlock;

impl Block for PinkConcreteBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:pink-concrete",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PinkConcreteBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-pink-concrete:block/pink_concrete"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PinkConcreteBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PinkConcreteBlock::RENDER;

pub struct BlockPinkConcreteMod;

impl BlockPinkConcreteMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
