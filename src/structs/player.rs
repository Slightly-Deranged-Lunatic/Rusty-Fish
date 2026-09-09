use std::collections::HashMap;

use crate::{enums::{biomes::Biome, inventory_item::InventoryItem, rarity::Rarity}, structs::fish::Fish};

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

    pub fn get_inventory_as_vec(&self) -> Vec<String> {
        // Vector is used to we can make a ListItem out of it
        let mut inventory_as_vec: Vec<String> = vec![];
        for (item, count) in &self.inventory {
            log::info!("{:?}", item);
            match item {
                InventoryItem::InvFish(item) => inventory_as_vec.push(format!("{} x {}", item.name.clone(), count).to_string()),
            }
        }
        return inventory_as_vec;
    }
}
