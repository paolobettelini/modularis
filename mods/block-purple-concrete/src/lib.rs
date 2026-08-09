use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PurpleConcreteBlock;

impl Block for PurpleConcreteBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:purple-concrete",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PurpleConcreteBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-purple-concrete:block/purple_concrete"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PurpleConcreteBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PurpleConcreteBlock::RENDER;

pub struct BlockPurpleConcreteMod;

impl BlockPurpleConcreteMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
