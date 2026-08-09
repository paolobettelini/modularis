use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct LightGrayConcreteBlock;

impl Block for LightGrayConcreteBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:light-gray-concrete",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for LightGrayConcreteBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-light-gray-concrete:block/light_gray_concrete"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = LightGrayConcreteBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = LightGrayConcreteBlock::RENDER;

pub struct BlockLightGrayConcreteMod;

impl BlockLightGrayConcreteMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
