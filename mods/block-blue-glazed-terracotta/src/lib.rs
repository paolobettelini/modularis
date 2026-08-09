use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct BlueGlazedTerracottaBlock;

impl Block for BlueGlazedTerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:blue-glazed-terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for BlueGlazedTerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-blue-glazed-terracotta:block/blue_glazed_terracotta"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = BlueGlazedTerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = BlueGlazedTerracottaBlock::RENDER;

pub struct BlockBlueGlazedTerracottaMod;

impl BlockBlueGlazedTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
