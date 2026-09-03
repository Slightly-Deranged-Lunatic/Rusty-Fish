#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default,
)]
pub enum Biome {
    #[default]
    BackyardPond,
}
