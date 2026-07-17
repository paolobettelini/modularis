use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct CalciteBlock;
impl Block for CalciteBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:calcite",
        is_air: false,
        solid: true,
        opaque: true,
    };
}
impl BlockRender for CalciteBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-calcite:block/calcite"),
        textures: None,
    };
}
pub const BLOCK_INFO: BlockInfo = CalciteBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CalciteBlock::RENDER;
pub struct BlockCalciteMod;
impl BlockCalciteMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
