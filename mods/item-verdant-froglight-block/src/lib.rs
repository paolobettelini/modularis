use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct VerdantFroglightBlockItem;

impl Item for VerdantFroglightBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:verdant_froglight_block",
        label: "Verdant Froglight",
    };
}

impl ItemRender for VerdantFroglightBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-verdant-froglight-block:item/verdant_froglight_block"),
    };
}

pub const ITEM_INFO: ItemInfo = VerdantFroglightBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <VerdantFroglightBlockItem as ItemRender>::RENDER;

pub struct ItemVerdantFroglightBlockMod;

impl ItemVerdantFroglightBlockMod {
    pub fn init(_block: &mut block_verdant_froglight::BlockVerdantFroglightMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
