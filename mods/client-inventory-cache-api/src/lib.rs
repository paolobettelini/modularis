use bevy::prelude::*;
use inventory_core_api::Inventory;

#[derive(Resource, Debug, Clone, Default)]
pub struct ClientInventoryCache {
    pub inventory: Option<Inventory>,
    pub selected_hotbar: u32,
    pub content_revision: u64,
    pub selection_revision: u64,
}

pub trait ClientInventoryCacheApi: Send + Sync + 'static {}
