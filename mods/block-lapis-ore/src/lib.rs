use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct LapisOreBlock;

impl Block for LapisOreBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:lapis-ore",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for LapisOreBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-lapis-ore:block/lapis_ore"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = LapisOreBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = LapisOreBlock::RENDER;

pub struct BlockLapisOreMod;

impl BlockLapisOreMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
