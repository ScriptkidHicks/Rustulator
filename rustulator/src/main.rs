//alright lets fuck around with some basic iced stuff
use iced::{
    widget::{button, column, row, text, text_input::Value, Button},
    window, Element,
};

#[derive(Debug)]
struct AppState {
    count: i32,
}

impl Default for AppState {
    fn default() -> Self {
        AppState { count: 0 }
    }
}

#[derive(Debug, Clone)]
enum Messages {
    Exit,
    Number(i64),
}

fn createKeyButton<'a>(buttonText: String, buttonAction: Messages) -> Element<'a, Messages> {
    button(text(buttonText).size(50))
        .on_press(buttonAction)
        .into()
}

fn update(state: &mut AppState, message: Messages) -> iced::Task<Messages> {
    match message {
        Messages::Exit => window::get_latest().and_then(window::close),
        _ => window::get_latest().and_then(window::close),
    }
}

fn view(state: &AppState) -> Element<Messages> {
    column![
        row![
            createKeyButton("7".to_string(), Messages::Number(7)),
            createKeyButton("8".to_string(), Messages::Number(8)),
            createKeyButton("9".to_string(), Messages::Number(9))
        ]
        .spacing(10),
        row![
            createKeyButton("4".to_string(), Messages::Number(4)),
            createKeyButton("5".to_string(), Messages::Number(5)),
            createKeyButton("6".to_string(), Messages::Number(6))
        ]
        .spacing(10),
        row![
            createKeyButton("1".to_string(), Messages::Number(1)),
            createKeyButton("2".to_string(), Messages::Number(2)),
            createKeyButton("3".to_string(), Messages::Number(3))
        ]
        .spacing(10),
    ]
    .spacing(10)
    .into()
}

fn main() -> iced::Result {
    iced::application("Calculator", update, view)
        .theme(|_s| iced::Theme::KanagawaDragon)
        .run()
}
