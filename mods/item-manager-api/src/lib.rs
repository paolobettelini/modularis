pub use generated_item_registry::ItemId;
use item_api::ItemInfo;
use item_render_api::ItemRenderInfo;

pub trait ItemManagerApi: Send + Sync + 'static {
    fn info(item: ItemId) -> &'static ItemInfo;
    fn render_info(item: ItemId) -> &'static ItemRenderInfo;
    fn all() -> &'static [ItemId];
    fn from_string(id: &str) -> Option<ItemId>;
    fn id(item: ItemId) -> &'static str;
    fn label(item: ItemId) -> &'static str;
}
