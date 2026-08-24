#[derive(Debug, Default)]
pub struct Player {
    difficulty: String
}

impl Player {
    pub fn new() -> Self {
        Player {
            difficulty: "Normal".to_owned()
        }
    }
}