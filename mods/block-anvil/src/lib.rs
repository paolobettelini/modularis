use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct AnvilBlock;

impl Block for AnvilBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:anvil",
        is_air: false,
        solid: true,
        opaque: false,
    };
}

impl BlockRender for AnvilBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-anvil:block/anvil"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = AnvilBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = AnvilBlock::RENDER;

pub struct BlockAnvilMod;

impl BlockAnvilMod {
    pub fn init(
        _template: &mut voxel_model_anvil_template_mod::VoxelModelAnvilTemplateMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
