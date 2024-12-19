#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::event::{self, Event};
use iced::widget::{button, center, checkbox, row, text, text_input, Column, Row, Text};
use iced::{window, Padding};
use iced::{Center, Element, Fill, Subscription, Task};
use rfd::FileDialog;

pub fn main() -> iced::Result {
    iced::application("Events - Iced", AppState::update, AppState::view)
        .subscription(AppState::subscription)
        // .exit_on_close_request(false)
        .run()
}

#[derive(Debug, Default)]
struct AppState {
    last: Vec<Event>,
    enabled: bool,
    source_directory: String,
    destination_directory: String,
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

impl AppState {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::EventOccurred(event) if self.enabled => {
                self.last.push(event);

                if self.last.len() > 5 {
                    let _ = self.last.remove(0);
                }

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
        let events = Column::with_children(
            self.last
                .iter()
                .map(|event| text!("{event:?}").size(40))
                .map(Element::from),
        );

        // center をつけると、余白領域を埋め尽くす
        let toggle =
            center(checkbox("Listen to runtime events", self.enabled).on_toggle(Message::Toggled));

        let exit = button(text("Exit").width(Fill).align_x(Center))
            .width(100)
            .padding(10)
            .on_press(Message::Exit);

        let source_dir_row = self.direction_row("Source Directory", &self.source_directory);

        let destination_dir_row =
            self.direction_row("Destination Directory", &self.destination_directory);

        let content = Column::new()
            .align_x(Center)
            .spacing(20)
            .push(source_dir_row)
            .push(destination_dir_row)
            .push(events)
            .push(toggle)
            .push(exit);

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
