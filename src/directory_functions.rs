use directories::ProjectDirs;
use reqwest;
use std::fs;
use std::path::{Path, PathBuf};

pub fn make_project_directories(project_directory: &ProjectDirs) {
    // Makes the directories the project needs for stuff
    // At the moment this is only a data directory, however, later on I plan on making a config directory.
    // The above is why this is a function and not just in download_words_list()

    let data_directory_children = vec!["words_lists", "logs"];
    for child in data_directory_children {
        let path = project_directory.data_dir().join(Path::new(child));
        // This will also make the parent directories so no need to worry about those
        // This isn't logged because the logger isn't initalized, however, I'm not *that* worried about it.
        // This function has a slim chance to fail anyways so
        match fs::create_dir_all(&path) {
            Ok(_) => {}
            Err(e) => panic!("Failed to make directory {:?} due to {}", &path, e),
        }
    }
}

pub fn download_words_list(project_directory: &ProjectDirs) {
    // Create project_directory.datadir()/words_list
    let words_list_directory = project_directory.data_dir().join(Path::new("words_lists"));
    let words_lists_files = vec!["easy_words.json", "normal_words.json", "hard_words.json"];
    let base_url = "https://raw.githubusercontent.com/Slightly-Deranged-Lunatic/Rusty-Fish/refs/heads/main/words_lists/";

    // Actually download the file from the repository
    for list in words_lists_files {
        let url = format!("{}{}", base_url, list);
        let response = reqwest::blocking::get(&url).unwrap();
        if !response.status().is_success() {
            let message = format!(
                "Response from {} was {} which was not a success. Is Github down?",
                &url,
                response.status()
            );
            log::error!("{}", message);
            panic!("{}", message);
        }

        let words_list_file = words_list_directory.join(Path::new(list));
        let _ = match fs::write(&words_list_file, response.text().unwrap()) {
            Ok(_) => log::info!("Successfully made file {:?}", words_list_file),
            Err(e) => {
                let message = format!("Failed to make file {:?} due to {}", words_list_file, e);
                log::error!("{}", message);
                panic!("{}", message);
            }
        };
    }
}

pub fn should_download_words_list(project_directory: &ProjectDirs) -> bool {
    // Determines if the word list files are there and whether or not they should be downloaded.
    // Returns true if we need to download the words lists again
    // Returns false if not

    let words_list_directory = project_directory.data_dir().join(Path::new("words_lists"));
    let files = fs::read_dir(words_list_directory).unwrap();
    let mut local_files: Vec<PathBuf> = Vec::new();
    for path in files {
        // Gets the last item in a directory
        let file = PathBuf::from(
            path.unwrap()
                .path()
                .components()
                .last()
                .unwrap()
                .as_os_str(),
        );
        local_files.push(file);
    }
    let mut expected_files = vec![
        PathBuf::from("easy_words.json"),
        PathBuf::from("normal_words.json"),
        PathBuf::from("hard_words.json"),
    ];

    expected_files.sort();
    local_files.sort();

    log::info!("local files on system: {:?}", local_files);
    log::info!("Expected files on system: {:?}", expected_files);
    if local_files == expected_files {
        return false;
    } else {
        return true;
    }
}
