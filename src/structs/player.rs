use std::collections::HashMap;

use crate::{enums::{biomes::Biome, inventory_item::InventoryItem, rarity::Rarity}, structs::fish::Fish};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Player {
    pub difficulty: String,
    pub inventory: HashMap<InventoryItem, i32>,
    pub last_caught_fish: Fish,
}

impl Player {
    pub fn new() -> Self {
        Player {
            difficulty: "normal".to_owned(),
            inventory: HashMap::new(),
            last_caught_fish: Fish::new("Trout".to_string(), vec![Biome::BackyardPond], Rarity::Common) // This is just placeholder information
        }
    }

    pub fn add_to_inventory(&mut self, item: InventoryItem) {
        log::info!("Adding {:#?} to inventory", item);
        let count = self.inventory.entry(item).or_insert(0);
        *count += 1;
    }
}
