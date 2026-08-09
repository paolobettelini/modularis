use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct RedConcretePowderBlock;

impl Block for RedConcretePowderBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:red-concrete-powder",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for RedConcretePowderBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-red-concrete-powder:block/red_concrete_powder"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = RedConcretePowderBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = RedConcretePowderBlock::RENDER;

pub struct BlockRedConcretePowderMod;

impl BlockRedConcretePowderMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
