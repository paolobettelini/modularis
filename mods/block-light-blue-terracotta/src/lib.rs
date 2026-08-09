use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct LightBlueTerracottaBlock;

impl Block for LightBlueTerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:light-blue-terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for LightBlueTerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-light-blue-terracotta:block/light_blue_terracotta"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = LightBlueTerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = LightBlueTerracottaBlock::RENDER;

pub struct BlockLightBlueTerracottaMod;

impl BlockLightBlueTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
