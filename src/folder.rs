use std::fs;


pub fn folder_exists(folder_name: &str) -> bool {
    fs::metadata(folder_name)
        .map(|metadata| metadata.is_dir())
        .unwrap_or(false)
}

pub fn create_folder(folder_name: &str) {
    fs::create_dir(folder_name)
        .expect("Failed to create folder");
}

pub fn set_folder(folder_name: &str){
    if !folder_exists(folder_name) {
        create_folder(folder_name);
    }
}