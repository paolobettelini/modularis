use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct MagentaConcreteBlock;

impl Block for MagentaConcreteBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:magenta-concrete",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for MagentaConcreteBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-magenta-concrete:block/magenta_concrete"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = MagentaConcreteBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = MagentaConcreteBlock::RENDER;

pub struct BlockMagentaConcreteMod;

impl BlockMagentaConcreteMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
