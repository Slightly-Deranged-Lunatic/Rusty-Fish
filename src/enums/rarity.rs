#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone, Copy)]
pub enum Rarity {
    #[default]
    Common,
    Rare,
}