use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct MagentaGlazedTerracottaBlock;

impl Block for MagentaGlazedTerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:magenta-glazed-terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for MagentaGlazedTerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-magenta-glazed-terracotta:block/magenta_glazed_terracotta"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = MagentaGlazedTerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = MagentaGlazedTerracottaBlock::RENDER;

pub struct BlockMagentaGlazedTerracottaMod;

impl BlockMagentaGlazedTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
