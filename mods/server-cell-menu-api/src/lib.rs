use audience_api::Audience;
use bevy::prelude::*;
use cell_menu_api::{CellMenuId, CellMenuState, CellMenuViewerId};
use inventory_core_api::{Inventory, InventoryCell, InventoryError, InventoryLayout};
use item_instance_api::ItemInstance;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct ServerCellMenu {
    pub menu: CellMenuState,
    viewers: HashSet<CellMenuViewerId>,
}

#[derive(Resource, Default)]
pub struct ServerCellMenus {
    menus: HashMap<CellMenuId, ServerCellMenu>,
}

impl ServerCellMenus {
    pub fn open_or_create(
        &mut self,
        viewer: CellMenuViewerId,
        id: CellMenuId,
        title: String,
        audience: Audience,
        layout: InventoryLayout,
    ) -> Result<CellMenuState, InventoryError> {
        if let Some(menu) = self.menus.get_mut(&id) {
            if can_open(&menu.menu.audience, viewer) {
                menu.viewers.insert(viewer);
                return Ok(menu.menu.clone());
            }
        }

        let inventory = Inventory::new(layout)?;
        let menu = CellMenuState {
            id: id.clone(),
            title,
            audience,
            inventory,
        };
        let mut viewers = HashSet::new();
        viewers.insert(viewer);
        self.menus.insert(
            id,
            ServerCellMenu {
                menu: menu.clone(),
                viewers,
            },
        );
        Ok(menu)
    }

    pub fn close(&mut self, viewer: CellMenuViewerId, id: &CellMenuId) -> bool {
        let Some(menu) = self.menus.get_mut(id) else {
            return false;
        };
        menu.viewers.remove(&viewer)
    }

    pub fn move_or_swap(
        &mut self,
        viewer: CellMenuViewerId,
        id: &CellMenuId,
        from: &InventoryCell,
        to: &InventoryCell,
    ) -> Result<Option<Vec<(InventoryCell, Option<ItemInstance>)>>, InventoryError> {
        let Some(menu) = self.menus.get_mut(id) else {
            return Ok(None);
        };
        if !can_interact(&menu.menu.audience, viewer, &menu.viewers) {
            return Ok(None);
        }
        if !menu.menu.inventory.move_or_swap(from, to)? {
            return Ok(None);
        }
        Ok(Some(vec![
            (from.clone(), menu.menu.inventory.get(from).cloned()),
            (to.clone(), menu.menu.inventory.get(to).cloned()),
        ]))
    }

    pub fn viewers(&self, id: &CellMenuId) -> Vec<CellMenuViewerId> {
        self.menus
            .get(id)
            .map(|menu| menu.viewers.iter().copied().collect())
            .unwrap_or_default()
    }

    pub fn menu_mut_for_viewer(
        &mut self,
        viewer: CellMenuViewerId,
        id: &CellMenuId,
    ) -> Option<&mut ServerCellMenu> {
        let menu = self.menus.get_mut(id)?;
        can_interact(&menu.menu.audience, viewer, &menu.viewers).then_some(menu)
    }
}

fn can_open(audience: &Audience, viewer: CellMenuViewerId) -> bool {
    match audience {
        Audience::Personal(owner) => *owner == viewer,
        Audience::Shared(_) => true,
    }
}

fn can_interact(
    audience: &Audience,
    viewer: CellMenuViewerId,
    viewers: &HashSet<CellMenuViewerId>,
) -> bool {
    match audience {
        Audience::Personal(owner) => *owner == viewer,
        Audience::Shared(_) => viewers.contains(&viewer),
    }
}

pub trait ServerCellMenuApi: Send + Sync + 'static {}
