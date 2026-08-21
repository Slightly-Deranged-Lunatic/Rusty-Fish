/// Module has all the code related to directories
use std::fs;
use std::path::Path;
use directories::ProjectDirs;
use reqwest;

pub fn make_project_directories(project_directory: & ProjectDirs) {
    // Makes the directories the project needs for stuff
    // At the moment this is only a data directory, however, later on I plan on making a config directory.
    // The above is why this is a function and not just in download_words_list()

    let words_list_directory = project_directory.data_dir().join(Path::new("words_list"));
    // This will also make the parent directories so no need to worry about those
    let _ = match fs::create_dir_all(&words_list_directory) {
        Ok(_) => println!("Sucessfully made directory {:?}", words_list_directory),
        Err(_) => println!("Directory {:?} failed to be made!", words_list_directory)
    };
}

pub fn download_words_list(project_directory: & ProjectDirs) {
    // Create project_directory.datadir()/words_list
    let words_list_directory = project_directory.data_dir().join(Path::new("words_list"));
    let words_lists_files = vec!["easy_words.json", "normal_words.json","hard_words.json"];
    let base_url = "https://raw.githubusercontent.com/Slightly-Deranged-Lunatic/Rusty-Fish/refs/heads/main/words_lists/";

    // Actually download the file from the repository
    for list in words_lists_files {
        let url = format!("{}{}", base_url, list);
        let response = reqwest::blocking::get(url).unwrap();

        let words_list_file = words_list_directory.join(Path::new(list));
        let _ = match fs::write(&words_list_file, response.text().unwrap()) {
            Ok(_) => println!("Successfully made file {:?}", words_list_file),
            Err(_) => println!("Failed to make file {:?}", words_list_file)
        };
    }
}