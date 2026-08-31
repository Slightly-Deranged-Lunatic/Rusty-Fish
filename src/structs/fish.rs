use crate::enums::{biomes::Biome, rarity::Rarity};

#[derive(Debug, Default)]
pub struct Fish {
    pub name: String,
    pub biome: Vec<Biome>,
    pub rarity: Rarity,
}

impl Fish {
    pub fn new(name: String, biome: Vec<Biome>, rarity: Rarity) -> Self {
        Fish {
            name: name,
            biome: biome,
            rarity: rarity
        }
    }
}
