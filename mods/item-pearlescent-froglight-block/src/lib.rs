use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PearlescentFroglightBlockItem;

impl Item for PearlescentFroglightBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:pearlescent_froglight_block",
        label: "Pearlescent Froglight",
    };
}

impl ItemRender for PearlescentFroglightBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-pearlescent-froglight-block:item/pearlescent_froglight_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PearlescentFroglightBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PearlescentFroglightBlockItem as ItemRender>::RENDER;

pub struct ItemPearlescentFroglightBlockMod;

impl ItemPearlescentFroglightBlockMod {
    pub fn init(_block: &mut block_pearlescent_froglight::BlockPearlescentFroglightMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
