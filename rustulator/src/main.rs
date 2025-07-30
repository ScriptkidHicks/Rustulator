use core::num;

//alright lets fuck around with some basic iced stuff
use iced::{
    widget::{
        button, column, row, shader::wgpu::naga::proc::Alignment, text, text_input::Value, Button,
    },
    window, Element,
};

#[derive(Debug)]
struct AppState {
    currentNumber: i128,
    inputNumber: i128,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            currentNumber: 0,
            inputNumber: 0,
        }
    }
}

#[derive(Debug, Clone)]
enum NumberActions {
    Multiply,
    Subtract,
    Add,
}

#[derive(Debug, Clone)]
enum Messages {
    NumberAction(NumberActions),
    Invert,
    Decimal,
    ClearEverything,
    Clear,
    Delete,
    Number(i128),
    Process,
    Percentage,
}

fn create_key_button<'a>(buttonText: String, buttonAction: Messages) -> Element<'a, Messages> {
    button(text(buttonText).size(30))
        .height(150)
        .width(150)
        .on_press(buttonAction)
        .into()
}

fn create_number_button<'a>(number: i128) -> Element<'a, Messages> {
    create_key_button(number.to_string(), Messages::Number(number))
}

fn update(state: &mut AppState, message: Messages) {
    match message {
        Messages::NumberAction(numberAction) => {
            //todo do some stuff
        }
        Messages::Number(number) => {
            state.inputNumber = (state.inputNumber * 10) + number;
            //todo do some stuff
        }
        Messages::Invert => {
            state.inputNumber = state.inputNumber * -1;
        }
        Messages::Decimal => {}
        Messages::Process => {}
        Messages::Clear => {
            state.inputNumber = 0;
        }
        Messages::ClearEverything => {
            state.currentNumber = 0;
            state.inputNumber = 0;
        }
        Messages::Delete => {}
        Messages::Percentage => {}
    }
}

fn view(state: &AppState) -> Element<Messages> {
    column![
        row![column![text(state.currentNumber), text(state.inputNumber)]],
        row![
            create_key_button("%".to_string(), Messages::Percentage),
            create_key_button("CE".to_string(), Messages::ClearEverything),
            create_key_button("C".to_string(), Messages::Clear),
            create_key_button("Del".to_string(), Messages::Delete)
        ]
        .spacing(10),
        row![
            create_number_button(7),
            create_number_button(8),
            create_number_button(9),
            create_key_button(
                "X".to_string(),
                Messages::NumberAction(NumberActions::Multiply)
            )
        ]
        .spacing(10),
        row![
            create_number_button(4),
            create_number_button(5),
            create_number_button(6),
            create_key_button(
                "-".to_string(),
                Messages::NumberAction(NumberActions::Subtract)
            )
        ]
        .spacing(10),
        row![
            create_number_button(1),
            create_number_button(2),
            create_number_button(3),
            create_key_button("+".to_string(), Messages::NumberAction(NumberActions::Add))
        ]
        .spacing(10),
        row![
            create_key_button("I".to_string(), Messages::Invert),
            create_number_button(0),
            create_key_button(".".to_string(), Messages::Decimal),
            create_key_button("=".to_string(), Messages::Process)
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
