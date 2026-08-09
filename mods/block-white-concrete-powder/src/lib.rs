use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct WhiteConcretePowderBlock;

impl Block for WhiteConcretePowderBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:white-concrete-powder",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for WhiteConcretePowderBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-white-concrete-powder:block/white_concrete_powder"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = WhiteConcretePowderBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = WhiteConcretePowderBlock::RENDER;

pub struct BlockWhiteConcretePowderMod;

impl BlockWhiteConcretePowderMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
