use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct TintedGlassBlockItem;

impl Item for TintedGlassBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:tinted_glass_block",
        label: "Tinted Glass",
    };
}

impl ItemRender for TintedGlassBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-tinted-glass-block:item/tinted_glass_block"),
    };
}

pub const ITEM_INFO: ItemInfo = TintedGlassBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <TintedGlassBlockItem as ItemRender>::RENDER;

pub struct ItemTintedGlassBlockMod;

impl ItemTintedGlassBlockMod {
    pub fn init(_block: &mut block_tinted_glass::BlockTintedGlassMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
