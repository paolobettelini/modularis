use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct StrippedBambooBlockBlock;

impl Block for StrippedBambooBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:stripped-bamboo-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for StrippedBambooBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-stripped-bamboo-block:block/stripped_bamboo_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = StrippedBambooBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = StrippedBambooBlockBlock::RENDER;

pub struct BlockStrippedBambooBlockMod;

impl BlockStrippedBambooBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
