use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct OrangeConcretePowderBlock;

impl Block for OrangeConcretePowderBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:orange-concrete-powder",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for OrangeConcretePowderBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-orange-concrete-powder:block/orange_concrete_powder"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = OrangeConcretePowderBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = OrangeConcretePowderBlock::RENDER;

pub struct BlockOrangeConcretePowderMod;

impl BlockOrangeConcretePowderMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
