/// Module has all the code related to directories
use std::fs;
use std::path::Path;
use directories::ProjectDirs;

pub fn make_project_directories(project_directory: ProjectDirs) {
    // Makes the directories the project needs for stuff
    // At the moment this is only a data directory, however, later on I plan on making a config directory.
    let project_directory = ProjectDirs::from("", "Deranged Lunatic Apps", "rusty-fish").unwrap();
    let words_list_directory = project_directory.data_dir().join(Path::new("words_list"));

    // This will also make the parent directories so no need to worry about those
    let _ = match fs::create_dir_all(&words_list_directory) {
        Ok(_) => println!("Sucessfully made directory {:?}", words_list_directory),
        Err(_) => println!("Directory {:?} failed to be made!", words_list_directory)
    };
}

pub fn download_words_list(project_directory: ProjectDirs) {
    // Create project_directory.datadir()/words_list
    let words_list_directory = project_directory.data_dir().join(Path::new("words_list"));
    let _ = match fs::create_dir(& words_list_directory) {
        Ok(_) => println!("Successfully made directory {:?}", words_list_directory),
        Err(_) => println!("Directory {:?} failed to be made!", words_list_directory)
    };

    
}