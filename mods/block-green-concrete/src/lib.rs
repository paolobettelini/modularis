use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct GreenConcreteBlock;

impl Block for GreenConcreteBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:green-concrete",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for GreenConcreteBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-green-concrete:block/green_concrete"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = GreenConcreteBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = GreenConcreteBlock::RENDER;

pub struct BlockGreenConcreteMod;

impl BlockGreenConcreteMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
