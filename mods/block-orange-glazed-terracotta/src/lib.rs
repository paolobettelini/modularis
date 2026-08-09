use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct OrangeGlazedTerracottaBlock;

impl Block for OrangeGlazedTerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:orange-glazed-terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for OrangeGlazedTerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-orange-glazed-terracotta:block/orange_glazed_terracotta"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = OrangeGlazedTerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = OrangeGlazedTerracottaBlock::RENDER;

pub struct BlockOrangeGlazedTerracottaMod;

impl BlockOrangeGlazedTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
