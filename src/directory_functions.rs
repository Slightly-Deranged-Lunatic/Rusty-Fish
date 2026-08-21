/// Module has all the code related to directories
use std::fs;
use std::path::Path;
use directories::ProjectDirs;

pub fn make_project_directory(project_directory: ProjectDirs) {
    // Makes the directories the project needs for stuff
    // At the moment this is only a data directory, however, later on I plan on making a config directory.
    let project_directory = ProjectDirs::from("", "Deranged Lunatic Apps", "rusty-fish").unwrap();
    let _ = match fs::create_dir_all(project_directory.data_dir()) {
        Ok(_) => println!("Sucessfully made directory {:?}", project_directory.data_dir()),
        Err(_) => println!("Directory {:?} failed to be made!", project_directory.data_dir())
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