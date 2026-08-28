#[derive(Debug, Default)]
pub struct Player {
    pub difficulty: String,
}

impl Player {
    pub fn new() -> Self {
        Player {
            difficulty: "normal".to_owned(),
        }
    }
}
