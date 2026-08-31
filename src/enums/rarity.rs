#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum Rarity {
    #[default]
    Common,
    Rare,
}