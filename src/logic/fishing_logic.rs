use directories::ProjectDirs;
use rand::seq::{IndexedRandom, SliceRandom};
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use std::{collections::HashMap, fmt::format, fs, io::BufReader};

use crate::{Player, enums::{
    biomes::Biome,
    rarity::Rarity,
},
    structs::fish::Fish
};

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
            log::error!(
                "Failed to the JSON data from {:?}, Error: {}",
                words_file,
                e
            );
            panic!();
        }
    };
    let words_json: JsonData = serde_json::from_str(&words).unwrap();
    let master_words_list = words_json.words;

    let mut rng = rand::rng();
    let words_list: Vec<String> = master_words_list.sample(&mut rng, 50).cloned().collect();

    // Convert the words_list to a vecctor of chars
    let mut char_list: Vec<char> = Vec::new();
    for word in words_list {
        let mut chars_word: Vec<char> = word.chars().collect();
        char_list.append(&mut chars_word);
        char_list.push(' ');
    }
    char_list.pop(); // Pop the last space so you don't have to press space at the end
    return char_list;
}

pub fn get_random_fish(project_directory: &ProjectDirs, player: &Player) -> Fish{
    let fish_list_path = project_directory.data_dir().join(format!("fishes.json"));
    let fishes_string = match fs::read_to_string(&fish_list_path) {
        Ok(file) => {
            log::info!("Successfully read the JSON data from {:?}", fish_list_path);
            file
        }
        Err(e) => {
            log::error!(
                "Failed to the JSON data from {:?}, Error: {}",
                fish_list_path,
                e
            );
            panic!();
        }
    };

     #[derive(Serialize, Deserialize, Debug)]
    struct Properties {
        biome: Vec<Biome>,
        rarity: Rarity
    }

    let fishes_hash: HashMap<String, Properties> = serde_json::from_str(&fishes_string).unwrap();
    let mut rng = rand::rng();

    let fish_vec = fishes_hash.keys().collect::<Vec<_>>();
    let random_fish = fish_vec.choose(&mut rng).unwrap().to_string();
    log::info!("Chosen fish {}", random_fish);
    let fish_properties = fishes_hash.get(&random_fish).unwrap();
    return Fish::new(random_fish, fish_properties.biome.clone(), fish_properties.rarity);
}