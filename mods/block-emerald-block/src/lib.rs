use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct EmeraldBlockBlock;

impl Block for EmeraldBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:emerald-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for EmeraldBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-emerald-block:block/emerald_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = EmeraldBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = EmeraldBlockBlock::RENDER;

pub struct BlockEmeraldBlockMod;

impl BlockEmeraldBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
