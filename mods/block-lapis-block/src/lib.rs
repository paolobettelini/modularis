use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct LapisBlockBlock;

impl Block for LapisBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:lapis-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for LapisBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-lapis-block:block/lapis_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = LapisBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = LapisBlockBlock::RENDER;

pub struct BlockLapisBlockMod;

impl BlockLapisBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
