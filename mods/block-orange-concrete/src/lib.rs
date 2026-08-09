use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct OrangeConcreteBlock;

impl Block for OrangeConcreteBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:orange-concrete",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for OrangeConcreteBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-orange-concrete:block/orange_concrete"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = OrangeConcreteBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = OrangeConcreteBlock::RENDER;

pub struct BlockOrangeConcreteMod;

impl BlockOrangeConcreteMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
