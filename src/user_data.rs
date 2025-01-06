use chrono::{DateTime, Local};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub name: String,
    pub last_edited: String,
    pub export_path: String,
    pub synced: bool,
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

pub fn is_valid_directory(directory_path: &String) -> bool {
    if directory_path.len() <= 1 {
        return false;
    }

    let path = Path::new(directory_path);
    path.is_dir() && fs::metadata(path).is_ok()
}

pub fn is_valid_file(file_path: &String) -> bool {
    let path = Path::new(file_path);
    path.is_file() && fs::metadata(path).is_ok()
}

// pub fn get_backup_filename(target_file: &String) -> String {
//     let metadata = Path::new(target_file).metadata().ok();
//     let last_edited: DateTime<Local> = metadata
//         .and_then(|m| m.modified().ok().map(DateTime::from))
//         .unwrap_or_else(Local::now);
//     let date = last_edited.format("%Y-%m-%d-%H-%M-%S").to_string();
//     format!("{}_{}", date, target_file)
// }

pub fn append_path(base: &String, path: &String) -> String {
    if base.len() == 0 {
        return path.clone();
    }

    if path.len() == 0 {
        return base.clone();
    }

    let base = base.trim_end_matches(|c| c == '/' || c == '\\');
    let path = path.trim_start_matches(|c| c == '/' || c == '\\');
    format!("{}/{}", base, path)
}

impl FileInfo {
    pub fn empty() -> Self {
        FileInfo {
            name: "".to_string(),
            last_edited: "".to_string(),
            export_path: "".to_string(),
            synced: false,
        }
    }

    pub fn new(name: String, modified: String, export_path: String) -> Self {
        FileInfo {
            name,
            last_edited: modified,
            export_path,
            synced: false,
        }
    }

    pub fn from_path(path: &Path) -> Self {
        let name = path.file_name().unwrap().to_str().unwrap().to_string();

        FileInfo {
            name,
            last_edited: Self::get_last_edited(path),
            export_path: "".to_string(),
            synced: false,
        }
    }

    pub fn refresh_last_edited(&mut self, self_directory: &String) {
        self.last_edited =
            Self::get_last_edited(&Path::new(&append_path(&self_directory, &self.name)));
    }

    fn get_last_edited(path: &Path) -> String {
        let metadata = path.metadata().ok();
        let last_edited: DateTime<Local> = metadata
            .and_then(|m| m.modified().ok().map(DateTime::from))
            .unwrap_or_else(Local::now);
        last_edited.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    pub fn backup_filename(&self) -> String {
        let date = self.last_edited.replace(" ", "-").replace(":", "-");
        format!("{}_{}", date, self.name)
    }

    pub fn refresh_synced(&mut self, backup_directory: &String) {
        let backup_synced = is_valid_directory(backup_directory)
            && is_valid_file(&append_path(backup_directory, &self.backup_filename()));
        // let export_synced = !is_valid_directory(&self.export_path)
        //     || is_valid_file(&append_path(&self.export_path, &self.name));
        self.synced = backup_synced;
    }

    pub fn sync(&mut self, self_directory: &String, backup_directory: &String) {
        let self_path = append_path(&self_directory, &self.name);

        if (is_valid_directory(backup_directory)) {
            let backup_path = append_path(&backup_directory, &self.backup_filename());
            fs::copy(&self_path, backup_path).ok();
        }

        if (is_valid_directory(&self.export_path)) {
            let export_path = append_path(&self.export_path, &self.name);
            fs::copy(&self_path, export_path).ok();
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

    pub fn refresh_synced(&mut self) {
        for file in self.files.iter_mut() {
            file.refresh_synced(&self.backup_directory);
        }
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
