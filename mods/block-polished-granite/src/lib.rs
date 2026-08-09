use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PolishedGraniteBlock;

impl Block for PolishedGraniteBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:polished-granite",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PolishedGraniteBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-polished-granite:block/polished_granite"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PolishedGraniteBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PolishedGraniteBlock::RENDER;

pub struct BlockPolishedGraniteMod;

impl BlockPolishedGraniteMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
