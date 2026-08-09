use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct DeepslateRedstoneOreBlock;

impl Block for DeepslateRedstoneOreBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:deepslate-redstone-ore",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for DeepslateRedstoneOreBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-deepslate-redstone-ore:block/deepslate_redstone_ore"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = DeepslateRedstoneOreBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = DeepslateRedstoneOreBlock::RENDER;

pub struct BlockDeepslateRedstoneOreMod;

impl BlockDeepslateRedstoneOreMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
