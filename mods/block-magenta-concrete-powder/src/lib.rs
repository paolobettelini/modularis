use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct MagentaConcretePowderBlock;

impl Block for MagentaConcretePowderBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:magenta-concrete-powder",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for MagentaConcretePowderBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-magenta-concrete-powder:block/magenta_concrete_powder"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = MagentaConcretePowderBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = MagentaConcretePowderBlock::RENDER;

pub struct BlockMagentaConcretePowderMod;

impl BlockMagentaConcretePowderMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
