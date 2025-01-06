use crate::app::{App, FileMessage, Message};
use crate::user_data::{is_valid_directory, DirectoryInfo, FileInfo};
use iced::widget::rule::Catalog;
use iced::widget::text::Shaping;
use iced::widget::text::Shaping::Advanced;
use iced::widget::text_input::Status;
use iced::widget::{
    button, center, horizontal_space, row, scrollable, text, text_input, Column, Row, Text,
};
use iced::{widget, Center, Element, Fill, Left, Padding, Theme};
fn text_input_style(is_valid: bool) -> fn(&Theme, text_input::Status) -> text_input::Style {
    if is_valid {
        text_input::default
    } else {
        |theme: &Theme, status: Status| {
            let mut style = text_input::default(theme, status);
            style.border.color = theme.palette().danger;
            style
        }
    }
}

impl App {
    pub fn view(&self) -> Element<Message> {
        let current_directory = self.user_data.find_directory(&self.current_directory);
        let file_list_elem = scrollable(if let Some(dir) = current_directory {
            let files = &dir.files;
            files
                .into_iter()
                .enumerate()
                .fold(Column::new(), |col, (index, file)| {
                    let file_row = Self::file_row_view(file)
                        .map(move |message| Message::FileMessage(index, message));
                    col.push(file_row)
                })
                .spacing(10)
                .width(Fill)
                .align_x(Left)
        } else {
            Column::new()
        })
        .height(Fill);

        // center をつけると、余白領域を埋め尽くす
        // let toggle =
        //     center(checkbox("Listen to runtime events", self.enabled).on_toggle(AppMessage::Toggled));

        // let exit = button(
        //     text("Exit \u{F17F3}")
        //         .shaping(Advanced)
        //         .width(Fill)
        //         .align_x(Center),
        // )
        // .width(100)
        // .padding(10)
        // .on_press(Message::Exit);

        let current_dir_elem = self.view_current_dir();

        let destination_dir_elem = self.view_backup_dir(current_directory);

        let content = widget::column![current_dir_elem, destination_dir_elem, file_list_elem]
            .align_x(Center)
            .spacing(20)
            .padding(20);
        center(content).into()
    }

    fn file_row_view(file: &FileInfo) -> Element<FileMessage> {
        let mut sync_button = button(
            text("\u{F1217}")
                .width(Fill)
                .align_x(Center)
                .shaping(Shaping::Advanced),
        )
        .width(50)
        .padding(10)
        .on_press(FileMessage::Sync);

        if !file.synced {
            sync_button = sync_button.style(button::success);
        } else {
            sync_button = sync_button.style(button::secondary)
        }

        row![
            sync_button,
            widget::column![
                widget::row![
                    Text::new(&file.name).shaping(Advanced),
                    horizontal_space(),
                    Text::new(&file.last_edited)
                ],
                widget::row![
                    Text::new("\u{F021D}")
                        .shaping(Advanced)
                        .style(text::primary),
                    text_input("(no export)", &file.export_path)
                        .padding(Padding::from([5, 10]))
                        .on_input(FileMessage::ExportPathInput)
                        .on_submit(FileMessage::ExportPathSubmit)
                ]
                .spacing(10)
                .padding(Padding::from([5, 0]))
                .align_y(Center)
            ]
        ]
        .align_y(Center)
        .spacing(10)
        .padding(Padding::from([5, 10]))
        .into()
    }

    fn view_current_dir(&self) -> Row<Message> {
        let open_directory_button = button(text("Current Directory".to_string()).align_x(Center))
            .width(200)
            .padding(10)
            .on_press(Message::CurrentDirectoryOpen);

        let directory_input = text_input("", &self.current_directory)
            .width(Fill)
            .padding(10)
            .style(text_input_style(self.current_directory_valid))
            .on_input(Message::CurrentDirectoryInput)
            .on_submit(Message::CurrentDirectorySubmit);

        row![open_directory_button, directory_input]
            .align_y(Center)
            .spacing(10)
            .padding(Padding::from([0, 20]))
    }

    fn view_backup_dir(&self, current_directory: Option<&DirectoryInfo>) -> Row<Message> {
        let open_directory_button = button(text("Backup Directory".to_string()).align_x(Center))
            .width(200)
            .padding(10)
            .on_press(Message::BackupDirectoryOpen);

        let backup_dir = if let Some(dir) = current_directory {
            &dir.backup_directory
        } else {
            &String::from("")
        };

        let backup_dir_valid = is_valid_directory(&backup_dir);
        let directory_input = text_input("", backup_dir)
            .width(Fill)
            .padding(10)
            .style(text_input_style(backup_dir_valid))
            .on_input(Message::BackupDirectoryInput)
            .on_submit(Message::BackupDirectorySubmit);

        row![open_directory_button, directory_input]
            .align_y(Center)
            .spacing(10)
            .padding(Padding::from([0, 20]))
    }
}
