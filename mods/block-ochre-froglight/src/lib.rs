use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct OchreFroglightBlock;

impl Block for OchreFroglightBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:ochre-froglight",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for OchreFroglightBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-ochre-froglight:block/ochre_froglight"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = OchreFroglightBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = OchreFroglightBlock::RENDER;

pub struct BlockOchreFroglightMod;

impl BlockOchreFroglightMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
