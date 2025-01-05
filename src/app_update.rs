use crate::app::{Message, App, FileMessage};
use crate::get_directory_of_file;
use crate::user_data::FileInfo;
use iced::{window, Event, Task};
use rfd::FileDialog;

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::EventOccurred(event) => {
                if let Event::Window(window::Event::CloseRequested) = event {
                    window::get_latest().and_then(window::close)
                } else if let Event::Window(window::Event::FileDropped(path)) = event {
                    let dir_path = get_directory_of_file(&path);
                    if dir_path.is_none() {
                        return Task::none();
                    }

                    self.source_directory = dir_path.unwrap().display().to_string();
                    let current_directory = self
                        .user_data
                        .touch_directory_or_insert(&self.source_directory);
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
            Message::DirectoryOpen => {
                Task::perform(async { FileDialog::new().pick_folder() }, |result| {
                    Message::DirectorySelected(result.map(|path| path.display().to_string()))
                })
            }
            Message::DirectorySelected(directory) => {
                self.source_directory = directory.unwrap_or_else(|| self.source_directory.clone());

                Task::none()
            }
            Message::SourceDirectoryInput(dir) => {
                self.source_directory = dir;

                Task::none()
            }
            Message::SourceDirectorySubmit => Task::none(),
            Message::FileMessage(index, file_message) => {
                let current_directory = self.user_data.touch_directory(&self.source_directory);
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
