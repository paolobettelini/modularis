use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct BrownTerracottaBlock;

impl Block for BrownTerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:brown-terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for BrownTerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-brown-terracotta:block/brown_terracotta"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = BrownTerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BrownTerracottaBlock::RENDER;

pub struct BlockBrownTerracottaMod;

impl BlockBrownTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
