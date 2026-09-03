use std::collections::HashMap;

use crate::enums::inventory_item::InventoryItem;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Player {
    pub difficulty: String,
    pub inventory: HashMap<InventoryItem, i32>,
}

impl Player {
    pub fn new() -> Self {
        Player {
            difficulty: "normal".to_owned(),
            inventory: HashMap::new(),
        }
    }

    pub fn add_to_inventory(&mut self, item: InventoryItem) {
        log::info!("Adding {:#?} to inventory", item);
        let count = self.inventory.entry(item).or_insert(0);
        *count += 1;
    }
}
