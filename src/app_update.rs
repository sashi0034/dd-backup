use crate::app::{App, FileMessage, Message};
use crate::get_directory_of_file;
use crate::user_data::{is_valid_directory, is_valid_file, FileInfo};
use iced::{window, Event, Task};
use rfd::FileDialog;
use std::path::PathBuf;

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::None => Task::none(),
            Message::EventOccurred(event) => {
                if let Event::Window(window::Event::CloseRequested) = event {
                    window::get_latest().and_then(window::close)
                } else if let Event::Window(window::Event::FileDropped(path)) = event {
                    if !is_valid_file(&path.to_str().unwrap_or("").to_string()) {
                        return Task::none();
                    }

                    let dir_path = get_directory_of_file(&path);
                    if dir_path.is_none() {
                        return Task::none();
                    }

                    self.change_current_directory(dir_path.unwrap().display().to_string());
                    let current_directory = self
                        .user_data
                        .touch_directory_or_insert(&self.current_directory);
                    current_directory.files.push(FileInfo::from_path(&path));

                    Task::none()
                } else {
                    Task::none()
                }
            }
            Message::Toggled(enabled) => {
                self.enabled = enabled;

                Task::none()
            }
            Message::CurrentDirectoryOpen => {
                Task::perform(async { FileDialog::new().pick_folder() }, |result| {
                    if let Some(path) = result {
                        return Message::CurrentDirectoryInput(path.display().to_string());
                    }

                    Message::None
                })
            }
            Message::CurrentDirectoryInput(dir) => {
                self.change_current_directory(dir);
                Task::none()
            }
            Message::CurrentDirectorySubmit => Task::none(),
            Message::BackupDirectoryOpen => {
                Task::perform(async { FileDialog::new().pick_folder() }, |result| {
                    if let Some(path) = result {
                        return Message::BackupDirectoryInput(path.display().to_string());
                    }

                    Message::None
                })
            }
            Message::BackupDirectoryInput(backup_dir) => {
                if !self.current_directory_valid {
                    return Task::none();
                }

                let current_directory = self
                    .user_data
                    .touch_directory_or_insert(&self.current_directory);
                current_directory.backup_directory = backup_dir;

                Task::none()
            }
            Message::BackupDirectorySubmit => Task::none(),
            Message::FileMessage(index, file_message) => {
                let current_directory = self.user_data.touch_directory(&self.current_directory);
                if let Some(dir) = current_directory {
                    if let Some(file) = dir.touch_file(index) {
                        match file_message {
                            FileMessage::ExportPathInput(path) => {
                                file.export_path = path;
                            }
                            FileMessage::ExportPathSubmit => {}
                        }
                    }
                }

                Task::none()
            }
            Message::Exit => window::get_latest().and_then(window::close),
        }
    }
}