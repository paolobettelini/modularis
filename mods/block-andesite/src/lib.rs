use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct AndesiteBlock;

impl Block for AndesiteBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:andesite",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for AndesiteBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-andesite:block/andesite"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = AndesiteBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = AndesiteBlock::RENDER;

pub struct BlockAndesiteMod;

impl BlockAndesiteMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
