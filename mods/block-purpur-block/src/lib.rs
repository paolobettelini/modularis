use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PurpurBlockBlock;

impl Block for PurpurBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:purpur-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PurpurBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-purpur-block:block/purpur_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PurpurBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PurpurBlockBlock::RENDER;

pub struct BlockPurpurBlockMod;

impl BlockPurpurBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
