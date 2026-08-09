use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct OchreFroglightBlockItem;

impl Item for OchreFroglightBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:ochre_froglight_block",
        label: "Ochre Froglight",
    };
}

impl ItemRender for OchreFroglightBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-ochre-froglight-block:item/ochre_froglight_block"),
    };
}

pub const ITEM_INFO: ItemInfo = OchreFroglightBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <OchreFroglightBlockItem as ItemRender>::RENDER;

pub struct ItemOchreFroglightBlockMod;

impl ItemOchreFroglightBlockMod {
    pub fn init(_block: &mut block_ochre_froglight::BlockOchreFroglightMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
