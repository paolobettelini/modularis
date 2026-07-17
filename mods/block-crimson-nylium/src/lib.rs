use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct CrimsonNyliumBlock;
impl Block for CrimsonNyliumBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:crimson-nylium",
        is_air: false,
        solid: true,
        opaque: true,
    };
}
impl BlockRender for CrimsonNyliumBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-crimson-nylium:block/crimson_nylium"),
        textures: None,
    };
}
pub const BLOCK_INFO: BlockInfo = CrimsonNyliumBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CrimsonNyliumBlock::RENDER;
pub struct BlockCrimsonNyliumMod;
impl BlockCrimsonNyliumMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
