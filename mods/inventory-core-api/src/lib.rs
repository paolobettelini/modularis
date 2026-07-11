use item_instance_api::ItemInstance;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InventorySectionId(pub String);

impl InventorySectionId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InventorySectionRole {
    Storage,
    Hotbar,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InventorySectionLayout {
    pub id: InventorySectionId,
    pub role: InventorySectionRole,
    pub columns: u32,
    pub cells: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct InventoryLayout {
    pub sections: Vec<InventorySectionLayout>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InventoryCell {
    pub section: InventorySectionId,
    pub index: u32,
}

impl InventoryCell {
    pub fn new(section: impl Into<String>, index: u32) -> Self {
        Self {
            section: InventorySectionId::new(section),
            index,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Inventory {
    pub layout: InventoryLayout,
    cells: HashMap<InventoryCell, ItemInstance>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InventoryError {
    DuplicateSection(InventorySectionId),
    InvalidColumns(InventorySectionId),
    CellOutsideLayout(InventoryCell),
}

impl InventoryLayout {
    pub fn validate(&self) -> Result<(), InventoryError> {
        let mut ids = HashSet::new();
        for section in &self.sections {
            if !ids.insert(section.id.clone()) {
                return Err(InventoryError::DuplicateSection(section.id.clone()));
            }
            if section.columns == 0 {
                return Err(InventoryError::InvalidColumns(section.id.clone()));
            }
        }
        Ok(())
    }

    pub fn section(&self, id: &InventorySectionId) -> Option<&InventorySectionLayout> {
        self.sections.iter().find(|section| &section.id == id)
    }

    pub fn hotbar(&self) -> Option<&InventorySectionLayout> {
        self.sections
            .iter()
            .find(|section| section.role == InventorySectionRole::Hotbar)
    }

    pub fn contains(&self, cell: &InventoryCell) -> bool {
        self.section(&cell.section)
            .is_some_and(|section| cell.index < section.cells)
    }
}

impl Inventory {
    pub fn new(layout: InventoryLayout) -> Result<Self, InventoryError> {
        layout.validate()?;
        Ok(Self {
            layout,
            cells: HashMap::new(),
        })
    }

    pub fn get(&self, cell: &InventoryCell) -> Option<&ItemInstance> {
        self.cells.get(cell)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&InventoryCell, &ItemInstance)> {
        self.cells.iter()
    }

    pub fn set(
        &mut self,
        cell: InventoryCell,
        item: Option<ItemInstance>,
    ) -> Result<Option<ItemInstance>, InventoryError> {
        if !self.layout.contains(&cell) {
            return Err(InventoryError::CellOutsideLayout(cell));
        }
        Ok(match item {
            Some(item) => self.cells.insert(cell, item),
            None => self.cells.remove(&cell),
        })
    }

    pub fn move_or_swap(
        &mut self,
        from: &InventoryCell,
        to: &InventoryCell,
    ) -> Result<bool, InventoryError> {
        if !self.layout.contains(from) {
            return Err(InventoryError::CellOutsideLayout(from.clone()));
        }
        if !self.layout.contains(to) {
            return Err(InventoryError::CellOutsideLayout(to.clone()));
        }
        if from == to || !self.cells.contains_key(from) {
            return Ok(false);
        }
        let source = self.cells.remove(from).expect("source was checked");
        let target = self.cells.remove(to);
        self.cells.insert(to.clone(), source);
        if let Some(target) = target {
            self.cells.insert(from.clone(), target);
        }
        Ok(true)
    }

    pub fn resize(&mut self, layout: InventoryLayout) -> Result<(), InventoryError> {
        layout.validate()?;
        self.cells.retain(|cell, _| layout.contains(cell));
        self.layout = layout;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use item_instance_api::ItemInstance;

    fn layout() -> InventoryLayout {
        InventoryLayout {
            sections: vec![InventorySectionLayout {
                id: InventorySectionId::new("hotbar"),
                role: InventorySectionRole::Hotbar,
                columns: 9,
                cells: 9,
            }],
        }
    }

    #[test]
    fn move_swaps_without_stack_semantics() {
        let mut inventory = Inventory::new(layout()).unwrap();
        let a = InventoryCell::new("hotbar", 0);
        let b = InventoryCell::new("hotbar", 1);
        let items = item_instance_api::all_items();
        inventory
            .set(a.clone(), Some(ItemInstance::new(items[0])))
            .unwrap();
        inventory
            .set(b.clone(), Some(ItemInstance::new(items[1])))
            .unwrap();

        assert!(inventory.move_or_swap(&a, &b).unwrap());
        assert_eq!(inventory.get(&a).unwrap().item, items[1]);
        assert_eq!(inventory.get(&b).unwrap().item, items[0]);
    }

    #[test]
    fn resize_discards_only_cells_outside_new_layout() {
        let mut inventory = Inventory::new(layout()).unwrap();
        let kept = InventoryCell::new("hotbar", 0);
        let removed = InventoryCell::new("hotbar", 8);
        let items = item_instance_api::all_items();
        inventory
            .set(kept.clone(), Some(ItemInstance::new(items[0])))
            .unwrap();
        inventory
            .set(removed.clone(), Some(ItemInstance::new(items[1])))
            .unwrap();
        let mut smaller = layout();
        smaller.sections[0].cells = 4;
        smaller.sections[0].columns = 4;

        inventory.resize(smaller).unwrap();

        assert!(inventory.get(&kept).is_some());
        assert!(inventory.get(&removed).is_none());
    }

    #[test]
    fn cbor_round_trip_preserves_items_in_cells() {
        let mut inventory = Inventory::new(layout()).unwrap();
        let cell = InventoryCell::new("hotbar", 0);
        let item = ItemInstance::new(item_instance_api::all_items()[0]);
        inventory.set(cell.clone(), Some(item.clone())).unwrap();

        let bytes = serde_cbor::to_vec(&inventory).unwrap();
        let decoded: Inventory = serde_cbor::from_slice(&bytes).unwrap();

        assert_eq!(decoded.get(&cell), Some(&item));
        assert_eq!(decoded, inventory);
    }
}
