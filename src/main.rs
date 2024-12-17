#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::event::{self, Event};
use iced::widget::{button, center, checkbox, text, Column, Row, Text};
use iced::window;
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
    selected_directory: Option<String>,
}

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(Event),
    Toggled(bool),
    DirectoryOpen,
    DirectorySelected(Option<String>),
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
                self.selected_directory = directory;

                Task::none()
            }
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

        let toggle = checkbox("Listen to runtime events", self.enabled).on_toggle(Message::Toggled);

        let exit = button(text("Exit").width(Fill).align_x(Center))
            .width(100)
            .padding(10)
            .on_press(Message::Exit);

        let dir_row = self.direction_row();

        let content = Column::new()
            .align_x(Center)
            .spacing(20)
            .push(dir_row)
            .push(events)
            .push(toggle)
            .push(exit);

        center(content).into()
    }

    fn direction_row(&self) -> Row<Message> {
        let open_directory_button = button(text("Open Directory").align_x(Center))
            .width(200)
            .padding(10)
            .on_press(Message::DirectoryOpen);

        let directory_text = Text::new(
            self.selected_directory
                .clone()
                .unwrap_or_else(|| "No directory selected".to_string()),
        )
        .width(400);

        let row = Row::new()
            .align_y(Center)
            .spacing(20)
            .push(open_directory_button)
            .push(directory_text);
        row
    }
}
