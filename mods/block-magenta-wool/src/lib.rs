use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct MagentaWoolBlock;

impl Block for MagentaWoolBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:magenta-wool",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for MagentaWoolBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-magenta-wool:block/magenta_wool"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = MagentaWoolBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = MagentaWoolBlock::RENDER;

pub struct BlockMagentaWoolMod;

impl BlockMagentaWoolMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
