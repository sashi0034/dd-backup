use crate::app::{App, FileMessage, Message};
use crate::user_data::FileInfo;
use iced::widget::text::Shaping;
use iced::widget::text::Shaping::Advanced;
use iced::widget::{
    button, center, horizontal_space, row, scrollable, text, text_input, Column, Row, Text,
};
use iced::{widget, Center, Element, Fill, Left, Padding};

impl App {
    pub fn view(&self) -> Element<Message> {
        let current_directory = self.user_data.find_directory(&self.source_directory);
        let file_list = scrollable(if let Some(dir) = current_directory {
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

        let destination_dir_row = if let Some(dir) = current_directory {
            self.direction_row("Destination Directory", &dir.backup_directory)
        } else {
            self.direction_row("Destination Directory", "")
        };

        let content = widget::column![source_dir_row, destination_dir_row, file_list, exit]
            .align_x(Center)
            .spacing(20)
            .padding(20);
        center(content).into()
    }

    fn file_row_view(file: &FileInfo) -> Element<FileMessage> {
        let sync_button = button(
            text("\u{F1217}")
                .width(Fill)
                .align_x(Center)
                .shaping(Shaping::Advanced),
        )
        .style(button::success)
        .width(50)
        .padding(10);
        // .on_press(AppMessage::Exit);

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
