use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct LightGrayConcretePowderBlock;

impl Block for LightGrayConcretePowderBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:light-gray-concrete-powder",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for LightGrayConcretePowderBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-light-gray-concrete-powder:block/light_gray_concrete_powder"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = LightGrayConcretePowderBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = LightGrayConcretePowderBlock::RENDER;

pub struct BlockLightGrayConcretePowderMod;

impl BlockLightGrayConcretePowderMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
