use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct GraniteBlock;

impl Block for GraniteBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:granite",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for GraniteBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-granite:block/granite"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = GraniteBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = GraniteBlock::RENDER;

pub struct BlockGraniteMod;

impl BlockGraniteMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
