use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct AmethystBlockBlock;

impl Block for AmethystBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:amethyst-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for AmethystBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-amethyst-block:block/amethyst_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = AmethystBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = AmethystBlockBlock::RENDER;

pub struct BlockAmethystBlockMod;

impl BlockAmethystBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
