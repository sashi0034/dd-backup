use crate::app::{App, FileMessage, Message};
use crate::get_directory_of_file;
use crate::save_data::{store_save_data, SAVE_PATH};
use crate::user_data::{is_valid_file, FileInfo};
use iced::{window, Event, Task};
use rfd::FileDialog;
use std::path::Path;
use std::process::Command;

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::None => Task::none(),
            Message::EventOccurred(event) => {
                if let Event::Window(window::Event::CloseRequested) = event {
                    store_save_data(&self);
                    window::get_latest().and_then(window::close)
                } else if let Event::Window(window::Event::FileDropped(path)) = event {
                    Task::done(Message::DropFile(path))
                } else {
                    Task::none()
                }
            }
            Message::DropFile(path) => {
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
                if self.current_directory_valid {
                    let current_directory_info = self
                        .user_data
                        .touch_directory_or_insert(&self.current_directory);
                    current_directory_info.refresh_files();
                }

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
                current_directory.refresh_files();

                Task::none()
            }
            Message::BackupDirectorySubmit => Task::none(),
            Message::FileMessage(index, file_message) => {
                let current_directory = self.user_data.touch_directory(&self.current_directory);
                if let Some(dir) = current_directory {
                    let dir_path = dir.path.clone();
                    let backup_directory = dir.backup_directory.clone();
                    if let Some(file) = dir.touch_file(index) {
                        match file_message {
                            FileMessage::IgnoreInput(_) => {}
                            FileMessage::Sync => {
                                file.refresh_last_edited(&dir_path);
                                file.sync(&dir_path, &backup_directory);
                                file.refresh_synced(&backup_directory);
                                file.refresh_export_valid();
                            }
                            FileMessage::ExportPathInput(path) => {
                                file.export_path = path;
                                file.refresh_export_valid();
                            }
                            FileMessage::ExportPathSubmit => {}
                            FileMessage::Remove => {
                                dir.files.remove(index);
                            }
                            FileMessage::RemoveAllowedToggled(allowed) => {
                                file.remove_allowed = allowed
                            }
                        }
                    }
                }

                Task::none()
            }
            Message::AddFileInCurrentDirectory => {
                Task::perform(async { FileDialog::new().pick_file() }, |result| {
                    if let Some(path) = result {
                        return Message::DropFile(path);
                    }

                    Message::None
                })
            }
            Message::OpenCurrentDirectory => {
                open_in_explorer(&self.current_directory).ok();
                Task::none()
            }
            Message::OpenSaveData => {
                open_in_explorer(SAVE_PATH).ok();
                Task::none()
            }
        }
    }
}

fn open_in_explorer(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        Command::new("explorer").arg(path).spawn()?;
    } else if cfg!(target_os = "macos") {
        Command::new("open").arg(path).spawn()?;
    } else if cfg!(target_os = "linux") {
        Command::new("xdg-open").arg(path).spawn()?;
    } else {
        return Err("Unsupported OS".into());
    }
    Ok(())
}
