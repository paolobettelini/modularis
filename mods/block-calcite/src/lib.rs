use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, BlockTextures, RenderShape};
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
        shape: RenderShape::Cube,
        textures: Some(BlockTextures::Uniform("block-calcite/calcite.png")),
    };
}
pub const BLOCK_INFO: BlockInfo = CalciteBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CalciteBlock::RENDER;
pub struct BlockCalciteMod;
impl BlockCalciteMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
