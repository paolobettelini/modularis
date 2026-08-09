use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct EmeraldOreBlock;

impl Block for EmeraldOreBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:emerald-ore",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for EmeraldOreBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-emerald-ore:block/emerald_ore"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = EmeraldOreBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = EmeraldOreBlock::RENDER;

pub struct BlockEmeraldOreMod;

impl BlockEmeraldOreMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
