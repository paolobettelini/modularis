use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PurpleConcretePowderBlock;

impl Block for PurpleConcretePowderBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:purple-concrete-powder",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PurpleConcretePowderBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-purple-concrete-powder:block/purple_concrete_powder"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PurpleConcretePowderBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PurpleConcretePowderBlock::RENDER;

pub struct BlockPurpleConcretePowderMod;

impl BlockPurpleConcretePowderMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
