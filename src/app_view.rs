use crate::app::FileMessage::RemoveAllowedToggled;
use crate::app::{App, FileMessage, Message};
use crate::user_data::{is_valid_directory, DirectoryInfo, FileInfo};
use iced::widget::rule::Catalog;
use iced::widget::text::Shaping;
use iced::widget::text::Shaping::Advanced;
use iced::widget::text_input::Status;
use iced::widget::{
    button, center, horizontal_rule, horizontal_space, row, scrollable, text, text_input, Column,
    Row, Text,
};
use iced::{widget, Center, Element, Fill, Left, Length, Padding, Theme};

fn text_input_style_by_status(
    is_valid: bool,
) -> fn(&Theme, text_input::Status) -> text_input::Style {
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

fn text_input_borderless_style(theme: &Theme, status: text_input::Status) -> text_input::Style {
    let mut style = text_input::default(theme, status);
    style.border.color = theme.palette().background;
    style
}

impl App {
    pub fn view(&self) -> Element<Message> {
        let current_directory_info = self.user_data.find_directory(&self.current_directory);

        // カレントディレクトリ
        let current_dir_elem = self.view_current_dir();

        // バックアップディレクトリ
        let backup_dir_elem = self.view_backup_dir(current_directory_info);

        let make_bottom_button = |text: &'static str, message: Message| {
            button(text.clone())
                .on_press(message)
                .style(button::primary)
                .width(Length::Shrink)
        };

        // ファイルリスト (底辺)
        let file_list_bottom = widget::column![
            horizontal_rule(0.5),
            widget::row![
                text("\u{F0966} Drop files here to backup")
                    .shaping(Advanced)
                    .style(text::secondary),
                horizontal_space(),
                make_bottom_button("Open Save Data", Message::OpenSaveData),
                make_bottom_button("Open Current Directory", Message::OpenCurrentDirectory),
                make_bottom_button("Add File", Message::AddFileInCurrentDirectory),
            ]
            .width(Fill)
            .align_y(Center)
            .spacing(10),
        ]
        .spacing(10);

        // ファイルリスト (本体)
        let file_list_elem = scrollable(if let Some(dir) = current_directory_info {
            let files = &dir.files;
            files
                .into_iter()
                .enumerate()
                .fold(Column::new(), |col, (index, file)| {
                    let file_row = Self::file_row_view(file)
                        .map(move |message| Message::FileMessage(index, message));
                    col.push(file_row)
                })
                .push(file_list_bottom)
                .spacing(10)
                .width(Fill)
                .align_x(Left)
        } else {
            Column::new().push(file_list_bottom)
        })
        .spacing(5)
        .height(Fill);

        let content = widget::column![current_dir_elem, backup_dir_elem, file_list_elem]
            .align_x(Center)
            .spacing(20)
            .padding(20);
        center(content).into()
    }

    fn file_row_view(file: &FileInfo) -> Element<FileMessage> {
        let mut sync_button = button(
            text("\u{F1378}")
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

        let remove_allowed_toggle = widget::toggler(file.remove_allowed)
            .text_shaping(Advanced)
            .on_toggle(RemoveAllowedToggled)
            .width(50);

        let remove_button = if file.remove_allowed {
            widget::row![
                remove_allowed_toggle,
                button(
                    text("\u{F0A7A}")
                        .width(Fill)
                        .align_x(Center)
                        .shaping(Advanced),
                )
                .width(50)
                .padding(10)
                .on_press(FileMessage::Remove)
                .style(button::danger),
            ]
            .align_y(Center)
        } else {
            widget::row![remove_allowed_toggle]
        }
        .spacing(10);

        row![
            sync_button,
            remove_button,
            widget::column![
                widget::row![
                    text_input("", &file.name)
                        .on_input(FileMessage::IgnoreInput)
                        .style(text_input_borderless_style),
                    horizontal_space(),
                    Text::new(&file.last_edited).style(text::primary)
                ],
                text_input("(no export)", &file.export_path)
                    .padding(Padding::from([5, 10]))
                    .style(text_input_style_by_status(
                        file.export_path.is_empty() || file.export_valid
                    ))
                    .on_input(FileMessage::ExportPathInput)
                    .on_submit(FileMessage::ExportPathSubmit)
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
            .style(text_input_style_by_status(self.current_directory_valid))
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
            .style(text_input_style_by_status(backup_dir_valid))
            .on_input(Message::BackupDirectoryInput)
            .on_submit(Message::BackupDirectorySubmit);

        row![open_directory_button, directory_input]
            .align_y(Center)
            .spacing(10)
            .padding(Padding::from([0, 20]))
    }
}
