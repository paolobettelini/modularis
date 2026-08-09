use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct LightBlueGlazedTerracottaBlock;

impl Block for LightBlueGlazedTerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:light-blue-glazed-terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for LightBlueGlazedTerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-light-blue-glazed-terracotta:block/light_blue_glazed_terracotta"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = LightBlueGlazedTerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = LightBlueGlazedTerracottaBlock::RENDER;

pub struct BlockLightBlueGlazedTerracottaMod;

impl BlockLightBlueGlazedTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
