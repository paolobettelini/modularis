use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct YellowWoolBlock;

impl Block for YellowWoolBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:yellow-wool",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for YellowWoolBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-yellow-wool:block/yellow_wool"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = YellowWoolBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = YellowWoolBlock::RENDER;

pub struct BlockYellowWoolMod;

impl BlockYellowWoolMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
