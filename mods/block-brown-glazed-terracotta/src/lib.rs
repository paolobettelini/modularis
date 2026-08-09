use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct BrownGlazedTerracottaBlock;

impl Block for BrownGlazedTerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:brown-glazed-terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for BrownGlazedTerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-brown-glazed-terracotta:block/brown_glazed_terracotta"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = BrownGlazedTerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BrownGlazedTerracottaBlock::RENDER;

pub struct BlockBrownGlazedTerracottaMod;

impl BlockBrownGlazedTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
