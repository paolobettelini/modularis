use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct WhiteConcreteBlock;

impl Block for WhiteConcreteBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:white-concrete",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for WhiteConcreteBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-white-concrete:block/white_concrete"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = WhiteConcreteBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = WhiteConcreteBlock::RENDER;

pub struct BlockWhiteConcreteMod;

impl BlockWhiteConcreteMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
