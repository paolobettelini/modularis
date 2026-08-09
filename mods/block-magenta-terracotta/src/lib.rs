use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct MagentaTerracottaBlock;

impl Block for MagentaTerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:magenta-terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for MagentaTerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-magenta-terracotta:block/magenta_terracotta"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = MagentaTerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = MagentaTerracottaBlock::RENDER;

pub struct BlockMagentaTerracottaMod;

impl BlockMagentaTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
