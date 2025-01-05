#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod app_update;
mod app_view;
mod user_data;

use crate::app::{App, Message};
use iced::event::{self};
use iced::Subscription;
use std::path::{Path, PathBuf};
use std::string::ToString;

pub fn main() -> iced::Result {
    iced::application("DD Backup", App::update, App::view)
        .subscription(App::subscription)
        .font(include_bytes!("../fonts/Noto_Sans_JP/NotoSansJP-VariableFont_wght.ttf").as_slice())
        .font(
            include_bytes!("../fonts/materialdesignicons/materialdesignicons-webfont.ttf")
                .as_slice(),
        )
        // .exit_on_close_request(false)
        .run_with(App::new)
}

fn get_directory_of_file(path: &Path) -> Option<PathBuf> {
    if path.is_dir() {
        None
    } else if path.is_file() {
        path.parent().map(|p| p.to_path_buf())
    } else {
        None
    }
}

impl App {
    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::EventOccurred)
    }
}
