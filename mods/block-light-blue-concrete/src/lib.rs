use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct LightBlueConcreteBlock;

impl Block for LightBlueConcreteBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:light-blue-concrete",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for LightBlueConcreteBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-light-blue-concrete:block/light_blue_concrete"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = LightBlueConcreteBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = LightBlueConcreteBlock::RENDER;

pub struct BlockLightBlueConcreteMod;

impl BlockLightBlueConcreteMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
