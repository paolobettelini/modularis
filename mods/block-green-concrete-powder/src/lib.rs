use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct GreenConcretePowderBlock;

impl Block for GreenConcretePowderBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:green-concrete-powder",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for GreenConcretePowderBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-green-concrete-powder:block/green_concrete_powder"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = GreenConcretePowderBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = GreenConcretePowderBlock::RENDER;

pub struct BlockGreenConcretePowderMod;

impl BlockGreenConcretePowderMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
