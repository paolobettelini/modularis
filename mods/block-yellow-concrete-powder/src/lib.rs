use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct YellowConcretePowderBlock;

impl Block for YellowConcretePowderBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:yellow-concrete-powder",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for YellowConcretePowderBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-yellow-concrete-powder:block/yellow_concrete_powder"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = YellowConcretePowderBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = YellowConcretePowderBlock::RENDER;

pub struct BlockYellowConcretePowderMod;

impl BlockYellowConcretePowderMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
