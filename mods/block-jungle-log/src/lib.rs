use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct JungleLogBlock;

impl Block for JungleLogBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:jungle-log",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for JungleLogBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-jungle-log:block/jungle_log"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = JungleLogBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = JungleLogBlock::RENDER;

pub struct BlockJungleLogMod;

impl BlockJungleLogMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
