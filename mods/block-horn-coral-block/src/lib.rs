use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct HornCoralBlockBlock;

impl Block for HornCoralBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:horn-coral-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for HornCoralBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-horn-coral-block:block/horn_coral_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = HornCoralBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = HornCoralBlockBlock::RENDER;

pub struct BlockHornCoralBlockMod;

impl BlockHornCoralBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
