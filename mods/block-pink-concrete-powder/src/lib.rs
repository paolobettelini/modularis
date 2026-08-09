use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PinkConcretePowderBlock;

impl Block for PinkConcretePowderBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:pink-concrete-powder",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PinkConcretePowderBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-pink-concrete-powder:block/pink_concrete_powder"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PinkConcretePowderBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PinkConcretePowderBlock::RENDER;

pub struct BlockPinkConcretePowderMod;

impl BlockPinkConcretePowderMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
