#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rarity {
    #[default]
    Common,
    Rare,
}