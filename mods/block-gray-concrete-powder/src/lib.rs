use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct GrayConcretePowderBlock;

impl Block for GrayConcretePowderBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:gray-concrete-powder",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for GrayConcretePowderBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-gray-concrete-powder:block/gray_concrete_powder"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = GrayConcretePowderBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = GrayConcretePowderBlock::RENDER;

pub struct BlockGrayConcretePowderMod;

impl BlockGrayConcretePowderMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
