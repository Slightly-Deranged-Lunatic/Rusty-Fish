use directories::ProjectDirs;
use rand::{
    distr::{Distribution, weighted::WeightedIndex},
    seq::IndexedRandom,
};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{collections::HashMap, fs};

use crate::{
    Player,
    enums::{biomes::Biome, rarity::Rarity},
    structs::{app::App, fish::Fish, fishing_minigame::FishingMinigame},
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

pub fn get_random_fish(project_directory: &ProjectDirs, app: &App) -> Fish {
    let fish_list_path = project_directory
        .data_dir()
        .join("fish_json")
        .join("fishes.json");
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
        rarity: Rarity,
    }

    let fishes_hash: HashMap<String, Properties> = serde_json::from_str(&fishes_string).unwrap();
    let mut rng = rand::rng();

    let weights: Vec<i8> = vec![Rarity::Common as i8, Rarity::Rare as i8];
    let rarities = vec![Rarity::Common, Rarity::Rare];
    let dist = WeightedIndex::new(weights).unwrap();
    let sampled_index = dist.sample(&mut rng);
    let selected_rarity = rarities[sampled_index];
    log::info!("Selected rarity is {:?}", selected_rarity);

    let fish_vec = fishes_hash.keys().collect::<Vec<_>>();
    let mut possible_fish: Vec<String> = vec![];
    for fish in &fish_vec {
        let fish_properties = fishes_hash.get(fish.to_owned()).unwrap();
        if fish_properties.biome.contains(&app.current_biome)
            && fish_properties.rarity == selected_rarity
        {
            possible_fish.push(fish.to_string());
        }
    }
    let random_fish = possible_fish.choose(&mut rng).unwrap().to_string();
    log::info!("Chosen fish {}", random_fish);
    let fish_properties = fishes_hash.get(&random_fish).unwrap();
    return Fish::new(
        random_fish,
        fish_properties.biome.clone(),
        fish_properties.rarity,
    );
}

pub fn calculate_statistics(fishing_minigame: &mut FishingMinigame) {
    // Calculates the WPM and accuracy of the player and sets the respective value in fishing_minigame
    fishing_minigame.elasped_time = fishing_minigame.start_time.elapsed().as_secs_f32();
    let mut typo_count: f32 = 0.0;
    for (index, character) in fishing_minigame.typed_text.iter().enumerate() {
        if fishing_minigame.words[index] != fishing_minigame.typed_text[index] {
            typo_count += 1.0;
        }
    }
    fishing_minigame.accuracy = (fishing_minigame.words.len() as f32 - typo_count)
        / fishing_minigame.words.len() as f32
        * 100.0;
    let words_typed = fishing_minigame.typed_text.len() as f32 / 5.0;
    fishing_minigame.wpm = (words_typed / (fishing_minigame.elasped_time / 60.0))
        * (fishing_minigame.accuracy / 100.0);
}
