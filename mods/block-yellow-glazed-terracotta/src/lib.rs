use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct YellowGlazedTerracottaBlock;

impl Block for YellowGlazedTerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:yellow-glazed-terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for YellowGlazedTerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-yellow-glazed-terracotta:block/yellow_glazed_terracotta"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = YellowGlazedTerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = YellowGlazedTerracottaBlock::RENDER;

pub struct BlockYellowGlazedTerracottaMod;

impl BlockYellowGlazedTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
