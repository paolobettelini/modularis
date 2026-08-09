use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct BrownConcretePowderBlock;

impl Block for BrownConcretePowderBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:brown-concrete-powder",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for BrownConcretePowderBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-brown-concrete-powder:block/brown_concrete_powder"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = BrownConcretePowderBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BrownConcretePowderBlock::RENDER;

pub struct BlockBrownConcretePowderMod;

impl BlockBrownConcretePowderMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
