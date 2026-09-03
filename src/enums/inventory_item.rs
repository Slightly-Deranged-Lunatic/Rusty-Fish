use crate::structs::fish;
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum InventoryItem {
    InvFish(fish::Fish)
}