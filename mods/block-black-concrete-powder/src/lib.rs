use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct BlackConcretePowderBlock;

impl Block for BlackConcretePowderBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:black-concrete-powder",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for BlackConcretePowderBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-black-concrete-powder:block/black_concrete_powder"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = BlackConcretePowderBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BlackConcretePowderBlock::RENDER;

pub struct BlockBlackConcretePowderMod;

impl BlockBlackConcretePowderMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
