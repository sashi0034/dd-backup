use std::path::PathBuf;
use crate::user_data::{is_valid_directory, UserData};
use iced::{Event, Task};

#[derive(Debug, Default)]
pub struct App {
    pub current_directory: String,
    pub current_directory_valid: bool,
    pub user_data: UserData,
}

#[derive(Debug, Clone)]
pub enum FileMessage {
    IgnoreInput(String),
    Sync,
    ExportPathInput(String),
    ExportPathSubmit,
    Remove,
    RemoveAllowedToggled(bool),
}

#[derive(Debug, Clone)]
pub enum Message {
    None,
    EventOccurred(Event),
    DropFile(PathBuf),
    CurrentDirectoryOpen,
    CurrentDirectoryInput(String),
    CurrentDirectorySubmit,
    BackupDirectoryOpen,
    BackupDirectoryInput(String),
    BackupDirectorySubmit,
    FileMessage(usize, FileMessage),
    AddFileInCurrentDirectory,
    OpenCurrentDirectory,
    OpenSaveData,
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                current_directory: "".to_string(),
                current_directory_valid: false,
                user_data: UserData::new(),
            },
            Task::none(),
        )
    }

    pub fn change_current_directory(&mut self, current_directory: String) {
        self.current_directory = current_directory;
        self.current_directory_valid = is_valid_directory(&self.current_directory);
    }
}
