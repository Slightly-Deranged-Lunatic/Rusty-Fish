#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rarity {
    #[default]
    Common = 90,
    Rare = 10,
}