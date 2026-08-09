use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct LightBlueConcretePowderBlock;

impl Block for LightBlueConcretePowderBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:light-blue-concrete-powder",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for LightBlueConcretePowderBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-light-blue-concrete-powder:block/light_blue_concrete_powder"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = LightBlueConcretePowderBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = LightBlueConcretePowderBlock::RENDER;

pub struct BlockLightBlueConcretePowderMod;

impl BlockLightBlueConcretePowderMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
