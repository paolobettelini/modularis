use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct LightBlueWoolBlock;

impl Block for LightBlueWoolBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:light-blue-wool",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for LightBlueWoolBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-light-blue-wool:block/light_blue_wool"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = LightBlueWoolBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = LightBlueWoolBlock::RENDER;

pub struct BlockLightBlueWoolMod;

impl BlockLightBlueWoolMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
