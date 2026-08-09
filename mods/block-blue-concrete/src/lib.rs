use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct BlueConcreteBlock;

impl Block for BlueConcreteBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:blue-concrete",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for BlueConcreteBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-blue-concrete:block/blue_concrete"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = BlueConcreteBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BlueConcreteBlock::RENDER;

pub struct BlockBlueConcreteMod;

impl BlockBlueConcreteMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
