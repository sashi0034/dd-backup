use crate::user_data::UserData;
use iced::{Event, Task};

#[derive(Debug, Default)]
pub struct App {
    pub enabled: bool,
    pub source_directory: String,
    pub user_data: UserData,
}

#[derive(Debug, Clone)]
pub enum FileMessage {
    ExportPathInput(String),
    ExportPathSubmit,
}

#[derive(Debug, Clone)]
pub enum Message {
    EventOccurred(Event),
    Toggled(bool),
    DirectoryOpen,
    DirectorySelected(Option<String>),
    SourceDirectoryInput(String),
    SourceDirectorySubmit,
    FileMessage(usize, FileMessage),
    Exit,
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                enabled: true,
                source_directory: "".to_string(),
                user_data: UserData::new(),
            },
            Task::none(),
        )
    }
}
