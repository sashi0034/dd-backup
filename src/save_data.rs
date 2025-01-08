use crate::app::App;
use crate::user_data::FileInfo;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct SaveFileData {
    name: String,
    export: String,
}

impl From<FileInfo> for SaveFileData {
    fn from(file_info: FileInfo) -> SaveFileData {
        SaveFileData {
            name: file_info.name.clone(),
            export: file_info.export_path.clone(),
        }
    }
}

impl Into<FileInfo> for SaveFileData {
    fn into(self) -> FileInfo {
        let mut file_info = FileInfo::empty();
        file_info.name = self.name;
        file_info.export_path = self.export;
        file_info
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct SaveDirectoryData {
    path: String,
    backup_directory: String,
    files: Vec<SaveFileData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SaveData {
    current_directory: String,
    directories: Vec<SaveDirectoryData>,
}

pub const SAVE_PATH: &str = "save.yaml";

pub fn store_save_data(app: &App) {
    let mut save_data = SaveData {
        current_directory: app.current_directory.clone(),
        directories: Vec::new(),
    };

    for dir in app.user_data.directories.iter() {
        let mut save_directory = SaveDirectoryData {
            path: dir.path.clone(),
            backup_directory: dir.backup_directory.clone(),
            files: Vec::new(),
        };

        for file in &dir.files {
            save_directory.files.push(SaveFileData::from(file.clone()));
        }

        save_data.directories.push(save_directory);
    }

    let yaml = serde_yaml::to_string(&save_data).unwrap();
    fs::write(SAVE_PATH, yaml).unwrap();
}

pub fn load_save_data() -> App {
    let content = fs::read_to_string(SAVE_PATH);
    if content.is_err() {
        return App::default();
    }

    let save_data: SaveData = serde_yaml::from_str(&content.unwrap()).unwrap();

    let mut app = App::default();
    app.change_current_directory(save_data.current_directory);
    for directory in save_data.directories {
        let dir_info = app.user_data.touch_directory_or_insert(&directory.path);
        dir_info.backup_directory = directory.backup_directory;
        for file in directory.files {
            let mut file_info: FileInfo = file.into();
            file_info.refresh_last_edited(&dir_info.path);
            file_info.refresh_synced(&dir_info.backup_directory);
            file_info.refresh_export_valid();
            dir_info.add_file(file_info);
        }

        dir_info.sort_files_by_last_edited();
    }

    app
}
