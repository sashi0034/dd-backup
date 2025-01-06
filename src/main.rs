#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod app_update;
mod app_view;
mod save_data;
mod user_data;

use crate::app::{App, Message};
use crate::save_data::load_save_data;
use iced::event::{self};
use iced::{Subscription, Task};
use std::env;
use std::path::{Path, PathBuf};
use std::string::ToString;

/// カレントディレクトリを実行ファイルのディレクトリに設定する関数
fn set_current_dir_to_executable_dir() -> Result<(), Box<dyn std::error::Error>> {
    let current_exe_path = env::current_exe()?;

    if let Some(exe_dir) = current_exe_path.parent() {
        env::set_current_dir(exe_dir)?;
        Ok(())
    } else {
        Err("Failed to determine the executable's directory".into())
    }
}

pub fn main() -> iced::Result {
    if let Err(e) = set_current_dir_to_executable_dir() {
        eprintln!("Error: {}", e);
    }

    iced::application("DD Backup", App::update, App::view)
        .subscription(App::subscription)
        .font(include_bytes!("../fonts/Noto_Sans_JP/NotoSansJP-VariableFont_wght.ttf").as_slice())
        .font(
            include_bytes!("../fonts/materialdesignicons/materialdesignicons-webfont.ttf")
                .as_slice(),
        )
        .exit_on_close_request(false)
        .run_with(|| (load_save_data(), Task::none()))
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
