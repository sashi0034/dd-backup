use chrono::{DateTime, Local};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub name: String,
    pub last_edited: String,
    pub export_path: String,
}

#[derive(Debug, Clone)]
pub struct DirectoryInfo {
    pub name: String,
    pub backup_directory: String,
    pub files: Vec<FileInfo>,
}

#[derive(Debug, Clone, Default)]
pub struct UserData {
    pub directories: Vec<DirectoryInfo>,
}

impl FileInfo {
    pub fn new(name: String, modified: String, export_path: String) -> Self {
        FileInfo {
            name,
            last_edited: modified,
            export_path,
        }
    }

    pub fn from_path(path: &Path) -> Self {
        let metadata = path.metadata().ok();
        let last_edited: DateTime<Local> = metadata
            .and_then(|m| m.modified().ok().map(DateTime::from))
            .unwrap_or_else(Local::now);
        let name = path.file_name().unwrap().to_str().unwrap().to_string();

        FileInfo {
            name,
            last_edited: last_edited.format("%Y-%m-%d %H:%M:%S").to_string(),
            export_path: "".to_string(),
        }
    }
}

impl DirectoryInfo {
    pub fn new(name: String, backup_directory: String) -> Self {
        DirectoryInfo {
            name,
            backup_directory,
            files: Vec::new(),
        }
    }

    pub fn add_file(&mut self, file: FileInfo) {
        self.files.push(file);
    }

    pub fn touch_file(&mut self, index: usize) -> Option<&mut FileInfo> {
        self.files.get_mut(index)
    }
}

impl UserData {
    pub fn new() -> Self {
        UserData {
            directories: Vec::new(),
        }
    }

    pub fn add_directory(&mut self, directory: DirectoryInfo) {
        self.directories.push(directory);
    }

    pub fn find_directory(&self, name: &str) -> Option<&DirectoryInfo> {
        self.directories.iter().find(|d| d.name == name)
    }

    pub fn touch_directory(&mut self, name: &str) -> Option<&mut DirectoryInfo> {
        self.directories.iter_mut().find(|d| d.name == name)
    }

    pub fn touch_directory_or_insert(&mut self, name: &str) -> &mut DirectoryInfo {
        if let Some(index) = self.directories.iter().position(|d| d.name == name) {
            &mut self.directories[index]
        } else {
            let directory = DirectoryInfo::new(name.to_string(), "".to_string());
            self.directories.push(directory);
            self.directories.last_mut().unwrap()
        }
    }
}
