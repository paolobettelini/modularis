use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct CyanWoolBlock;

impl Block for CyanWoolBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:cyan-wool",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for CyanWoolBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-cyan-wool:block/cyan_wool"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = CyanWoolBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CyanWoolBlock::RENDER;

pub struct BlockCyanWoolMod;

impl BlockCyanWoolMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
