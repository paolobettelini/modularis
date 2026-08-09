use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct StrippedJungleLogBlock;

impl Block for StrippedJungleLogBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:stripped-jungle-log",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for StrippedJungleLogBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-stripped-jungle-log:block/stripped_jungle_log"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = StrippedJungleLogBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = StrippedJungleLogBlock::RENDER;

pub struct BlockStrippedJungleLogMod;

impl BlockStrippedJungleLogMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
