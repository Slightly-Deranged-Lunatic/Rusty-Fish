use std::time::Instant;

use crate::{structs::fish::Fish};
pub struct FishingMinigame {
    pub typed_text: Vec<char>,
    pub start_time: Instant,
    pub words: Vec<char>,
    pub elasped_time: f32,
    pub accuracy: f32,
    pub wpm: f32,
    pub current_line: usize,
    pub position_in_line: i32,
    pub catch: Fish
}

impl FishingMinigame {
    pub fn new(words: Vec<char>, catch: Fish) -> Self {
        FishingMinigame {
            typed_text: Vec::new(),
            start_time: Instant::now(),
            words: words,
            elasped_time: 0.0,
            accuracy: 0.0,
            wpm: 0.0,
            current_line: 0,
            position_in_line: 0,
            catch: catch,
        }
    }

    pub fn insert_text(&mut self, character: char) {
        self.typed_text.push(character);
        self.position_in_line += 1;
        
    }

    pub fn pop_typed_text(&mut self) {
        self.typed_text.pop();
        self.position_in_line -= 1;
    }

    pub fn clear_typed_text(&mut self) {
        self.typed_text.clear();
    }
}