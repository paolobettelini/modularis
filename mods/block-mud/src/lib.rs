use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct MudBlock;

impl Block for MudBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:mud",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for MudBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-mud:block/mud"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = MudBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = MudBlock::RENDER;

pub struct BlockMudMod;

impl BlockMudMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
