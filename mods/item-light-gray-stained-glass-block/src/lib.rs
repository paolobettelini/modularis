use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct LightGrayStainedGlassBlockItem;

impl Item for LightGrayStainedGlassBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:light_gray_stained_glass_block",
        label: "Light Gray Stained Glass",
    };
}

impl ItemRender for LightGrayStainedGlassBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-light-gray-stained-glass-block:item/light_gray_stained_glass_block"),
    };
}

pub const ITEM_INFO: ItemInfo = LightGrayStainedGlassBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <LightGrayStainedGlassBlockItem as ItemRender>::RENDER;

pub struct ItemLightGrayStainedGlassBlockMod;

impl ItemLightGrayStainedGlassBlockMod {
    pub fn init(
        _block: &mut block_light_gray_stained_glass::BlockLightGrayStainedGlassMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
