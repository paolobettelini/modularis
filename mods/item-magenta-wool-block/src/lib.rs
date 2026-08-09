use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct MagentaWoolBlockItem;

impl Item for MagentaWoolBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:magenta_wool_block",
        label: "Magenta Wool",
    };
}

impl ItemRender for MagentaWoolBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-magenta-wool-block:item/magenta_wool_block"),
    };
}

pub const ITEM_INFO: ItemInfo = MagentaWoolBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <MagentaWoolBlockItem as ItemRender>::RENDER;

pub struct ItemMagentaWoolBlockMod;

impl ItemMagentaWoolBlockMod {
    pub fn init(_block: &mut block_magenta_wool::BlockMagentaWoolMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
