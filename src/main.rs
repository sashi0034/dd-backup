#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::{DateTime, Local, Utc};
use iced::event::{self, Event};
use iced::widget::text::Shaping::Advanced;
use iced::widget::text::{Shaping, Style};
use iced::widget::{
    button, center, checkbox, horizontal_space, row, scrollable, text, text_input, Column, Row,
    Text,
};
use iced::{widget, window, Left, Padding};
use iced::{Center, Element, Fill, Subscription, Task};
use rfd::FileDialog;
use std::fs;
use std::path::Path;
use std::string::ToString;
use std::time::SystemTime;

pub fn main() -> iced::Result {
    iced::application("DD Backup", App::update, App::view)
        .subscription(App::subscription)
        .font(include_bytes!("../fonts/Noto_Sans_JP/NotoSansJP-VariableFont_wght.ttf").as_slice())
        .font(include_bytes!("../fonts/materialdesignicons/materialdesignicons-webfont.ttf").as_slice())
        // .exit_on_close_request(false)
        .run_with(App::new)
}

#[derive(Debug, Default)]
struct App {
    enabled: bool,
    current_files: Vec<FileData>,
    source_directory: String,
    destination_directory: String,
}

#[derive(Debug, Clone)]
struct FileData {
    name: String,
    last_edited: String,
    export_path: String,
}

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(Event),
    Toggled(bool),
    DirectoryOpen,
    DirectorySelected(Option<String>),
    SourceDirectoryInput(String),
    SourceDirectorySubmit,
    Exit,
}

fn get_file_list_in_directory(dir_path: &str) -> Vec<FileData> {
    let path = Path::new(dir_path);

    if path.is_dir() {
        match fs::read_dir(path) {
            Ok(entries) => entries
                .filter_map(|entry| entry.ok())
                .filter_map(|entry| {
                    let metadata = entry.metadata().ok()?;
                    if !metadata.is_file() {
                        return None;
                    }

                    let last_edited: DateTime<Local> = DateTime::from(metadata.modified().ok()?);
                    let name = entry.path().file_name()?.to_str()?.to_string();
                    Some(FileData {
                        name,
                        last_edited: last_edited.format("%Y/%m/%d %H:%M:%S").to_string(),
                        export_path: "".to_string(),
                    })
                })
                .collect(),
            Err(_) => Vec::new(),
        }
    } else {
        Vec::new()
    }
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                enabled: true,
                current_files: Vec::new(),
                source_directory: "F:\\".to_string(),
                destination_directory: "".to_string(),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::EventOccurred(event) if self.enabled => {
                // self.last.push(event);
                //
                // if self.last.len() > 5 {
                //     let _ = self.last.remove(0);
                // }

                Task::none()
            }
            Message::EventOccurred(event) => {
                if let Event::Window(window::Event::CloseRequested) = event {
                    window::get_latest().and_then(window::close)
                } else if let Event::Window(window::Event::FileDropped(path)) = event {
                    self.source_directory = path.display().to_string();
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
                self.current_files = get_file_list_in_directory(&self.source_directory);

                Task::none()
            }
            Message::SourceDirectorySubmit => Task::none(),
            Message::Exit => window::get_latest().and_then(window::close),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::EventOccurred)
    }

    fn view(&self) -> Element<Message> {
        // let events = Column::with_children(
        //     self.last
        //         .iter()
        //         .map(|event| text!("{event:?}").size(40))
        //         .map(Element::from),
        // );

        let current_files = &self.current_files;
        let file_list = scrollable(
            current_files
                .into_iter()
                .fold(Column::new(), |col, file| {
                    let sync_button = button(
                        text("Sync")
                            .width(Fill)
                            .align_x(Center)
                            .shaping(Shaping::Advanced),
                    )
                    .style(button::success)
                    .width(80)
                    .padding(10);
                    // .on_press(Message::Exit);

                    col.push(
                        row![
                            sync_button,
                            widget::column![
                               widget::row![Text::new(&file.name).shaping(Advanced),
                                    horizontal_space(),
                                    Text::new(&file.last_edited)],
                                 widget::row![text_input("", &self.destination_directory) // FIXME
                                    .on_input(Message::SourceDirectoryInput)
                                    .on_submit(Message::SourceDirectorySubmit)]
                                .spacing(10)
                                .padding(Padding::from([5, 10]))
                            ]
                        ]
                        .align_y(Center)
                        .spacing(10)
                        .padding(Padding::from([5, 10])),
                    )
                })
                .spacing(10)
                .width(Fill)
                .align_x(Left),
        )
        .height(Fill);

        // center をつけると、余白領域を埋め尽くす
        // let toggle =
        //     center(checkbox("Listen to runtime events", self.enabled).on_toggle(Message::Toggled));

        let exit = button(
            text("Exit \u{F17F3}")
                .shaping(Advanced)
                .width(Fill)
                .align_x(Center),
        )
        .width(100)
        .padding(10)
        .on_press(Message::Exit);

        let source_dir_row = self.direction_row("Source Directory", &self.source_directory);

        let destination_dir_row =
            self.direction_row("Destination Directory", &self.destination_directory);

        let content = widget::column![source_dir_row, destination_dir_row, file_list, exit]
            .align_x(Center)
            .spacing(20)
            .padding(20);
        center(content).into()
    }

    fn direction_row(&self, button_text: &str, dir_str: &str) -> Row<Message> {
        let open_directory_button = button(text(button_text.to_string()).align_x(Center))
            .width(200)
            .padding(10)
            .on_press(Message::DirectoryOpen);

        // let directory_text = Text::new(
        //     self.selected_directory
        //         .clone()
        //         .unwrap_or_else(|| "(Nothing selected)".to_string()),
        // )
        // .width(400);

        let directory_input = text_input("", &dir_str)
            .width(Fill)
            .padding(10)
            .on_input(Message::SourceDirectoryInput)
            .on_submit(Message::SourceDirectorySubmit);

        row![open_directory_button, directory_input]
            .align_y(Center)
            .spacing(10)
            .padding(Padding::from([0, 20]))
    }
}
