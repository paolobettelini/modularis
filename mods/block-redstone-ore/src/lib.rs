use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct RedstoneOreBlock;

impl Block for RedstoneOreBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:redstone-ore",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for RedstoneOreBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-redstone-ore:block/redstone_ore"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = RedstoneOreBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = RedstoneOreBlock::RENDER;

pub struct BlockRedstoneOreMod;

impl BlockRedstoneOreMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
