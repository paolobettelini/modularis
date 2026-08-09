use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct GrayStainedGlassBlockItem;

impl Item for GrayStainedGlassBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:gray_stained_glass_block",
        label: "Gray Stained Glass",
    };
}

impl ItemRender for GrayStainedGlassBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-gray-stained-glass-block:item/gray_stained_glass_block"),
    };
}

pub const ITEM_INFO: ItemInfo = GrayStainedGlassBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <GrayStainedGlassBlockItem as ItemRender>::RENDER;

pub struct ItemGrayStainedGlassBlockMod;

impl ItemGrayStainedGlassBlockMod {
    pub fn init(_block: &mut block_gray_stained_glass::BlockGrayStainedGlassMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
