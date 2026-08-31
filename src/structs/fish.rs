use crate::enums::{biomes::Biome, rarity::Rarity};

#[derive(Debug, Default)]
pub struct Fish {
    pub name: String,
    pub biome: Vec<Biome>,
    pub rarity: Rarity,
}