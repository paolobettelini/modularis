use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct AncientDebrisBlockItem;

impl Item for AncientDebrisBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:ancient_debris_block",
        label: "Ancient Debris",
    };
}

impl ItemRender for AncientDebrisBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-ancient-debris-block:item/ancient_debris_block"),
    };
}

pub const ITEM_INFO: ItemInfo = AncientDebrisBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <AncientDebrisBlockItem as ItemRender>::RENDER;

pub struct ItemAncientDebrisBlockMod;

impl ItemAncientDebrisBlockMod {
    pub fn init(_block: &mut block_ancient_debris::BlockAncientDebrisMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
