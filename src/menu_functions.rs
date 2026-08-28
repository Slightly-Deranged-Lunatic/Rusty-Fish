use crate::{app::App, player::Player, ui};
use directories::ProjectDirs;
use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use ratatui::Frame;

pub fn do_action(selection: usize, app: &mut App) {
    let action = &app.list_items[selection];
    log::info!(
        "User selected choice #{} which correspondend to {}",
        selection,
        action
    );
    log::info!("Here is the entire list: {:?}", app.list_items);
    if action == "Fish" {
        app.fish();
    }
}

pub fn fish(project_directory: &ProjectDirs, player: & mut Player, app: &mut App, frame: &mut Frame<'_>, words: Vec<char>) {
    ui::render_fishing_ui(app, frame, words, player);
}

pub fn get_random_words(project_directory: &ProjectDirs, player: &Player) -> Vec<char> {
    #[derive(Serialize, Deserialize)]
    struct JsonData {
        words: Vec<String>,
    }

    let file_directory = project_directory.data_dir().join("words_lists");
    let words_file = file_directory.join(format!("{}_words.json", player.difficulty));
    let words = match fs::read_to_string(&words_file) {
        Ok(file) => {
            log::info!("Successfully read the JSON data from {:?}", words_file);
            file
        }
        Err(e) => {
            log::error!("Failed to the JSON data from {:?}, Error: {}", words_file, e);
            panic!();
        }
    };
    let words_json: JsonData = serde_json::from_str(&words).unwrap();
    let master_words_list = words_json.words;

    let mut rng = rand::rng();
    let words_list:Vec<String> = master_words_list.sample(&mut rng, 50).cloned().collect();
    
    // Convert the words_list to a vecctor of chars
    let mut char_list:Vec<char> = Vec::new();
    for word in words_list {
        let mut chars_word: Vec<char> = word.chars().collect();
        char_list.append(&mut chars_word);
        char_list.push(' ');
    }
    return char_list;
}
