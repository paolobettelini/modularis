use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct BrownConcreteBlock;

impl Block for BrownConcreteBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:brown-concrete",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for BrownConcreteBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-brown-concrete:block/brown_concrete"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = BrownConcreteBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BrownConcreteBlock::RENDER;

pub struct BlockBrownConcreteMod;

impl BlockBrownConcreteMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
