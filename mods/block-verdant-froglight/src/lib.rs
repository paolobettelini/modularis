use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct VerdantFroglightBlock;

impl Block for VerdantFroglightBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:verdant-froglight",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for VerdantFroglightBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-verdant-froglight:block/verdant_froglight"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = VerdantFroglightBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = VerdantFroglightBlock::RENDER;

pub struct BlockVerdantFroglightMod;

impl BlockVerdantFroglightMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
