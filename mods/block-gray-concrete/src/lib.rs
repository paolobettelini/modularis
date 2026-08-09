use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct GrayConcreteBlock;

impl Block for GrayConcreteBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:gray-concrete",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for GrayConcreteBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-gray-concrete:block/gray_concrete"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = GrayConcreteBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = GrayConcreteBlock::RENDER;

pub struct BlockGrayConcreteMod;

impl BlockGrayConcreteMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
