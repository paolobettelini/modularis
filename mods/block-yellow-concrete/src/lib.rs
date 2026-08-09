use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct YellowConcreteBlock;

impl Block for YellowConcreteBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:yellow-concrete",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for YellowConcreteBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-yellow-concrete:block/yellow_concrete"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = YellowConcreteBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = YellowConcreteBlock::RENDER;

pub struct BlockYellowConcreteMod;

impl BlockYellowConcreteMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
