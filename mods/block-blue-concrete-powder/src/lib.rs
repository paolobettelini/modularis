use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct BlueConcretePowderBlock;

impl Block for BlueConcretePowderBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:blue-concrete-powder",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for BlueConcretePowderBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-blue-concrete-powder:block/blue_concrete_powder"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = BlueConcretePowderBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BlueConcretePowderBlock::RENDER;

pub struct BlockBlueConcretePowderMod;

impl BlockBlueConcretePowderMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
