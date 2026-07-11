use bevy::prelude::*;
use cell_menu_api::{CellMenuId, CellMenuState};
use std::collections::HashMap;

#[derive(Resource, Debug, Clone, Default)]
pub struct ClientCellMenuCache {
    pub menus: HashMap<CellMenuId, CellMenuState>,
    pub active: Option<CellMenuId>,
    pub content_revision: u64,
}

pub trait ClientCellMenuCacheApi: Send + Sync + 'static {}
