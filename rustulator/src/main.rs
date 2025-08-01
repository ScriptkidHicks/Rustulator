use core::num;

//alright lets fuck around with some basic iced stuff
use iced::{
    border::width,
    widget::{button, column, row, text, text_input::Value, Button},
    window, Alignment, Element, Length,
};

#[derive(Debug)]
struct AppState {
    current_number: i128,
    input_number: i128,
    prepared_action: NumberActions,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            current_number: 0,
            input_number: 0,
            prepared_action: NumberActions::DoNothing,
        }
    }
}

#[derive(Debug, Clone)]
enum NumberActions {
    Multiply,
    Subtract,
    Add,
    DoNothing,
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

fn create_key_button<'a>(button_text: String, button_action: Messages) -> Element<'a, Messages> {
    button(
        text(button_text)
            .size(30)
            .height(Length::Fill)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .height(Length::Fill)
    .width(Length::Fill)
    .on_press(button_action)
    .into()
}

fn create_number_button<'a>(number: i128) -> Element<'a, Messages> {
    create_key_button(number.to_string(), Messages::Number(number))
}

fn update(state: &mut AppState, message: Messages) {
    match message {
        Messages::NumberAction(numberAction) => {
            state.prepared_action = numberAction;
        }
        Messages::Number(number) => {
            state.input_number = (state.input_number * 10) + number;
            //todo do some stuff
        }
        Messages::Invert => {
            state.input_number = state.input_number * -1;
        }
        Messages::Decimal => {}
        Messages::Process => match state.prepared_action {
            NumberActions::Add => {
                state.current_number += state.input_number;
            }
            NumberActions::Subtract => {
                state.current_number -= state.input_number;
            }
            NumberActions::Multiply => {
                state.current_number *= state.input_number;
            }
            NumberActions::DoNothing => {
                state.current_number = state.input_number;
                state.input_number = 0;
            }
        },
        Messages::Clear => {
            state.input_number = 0;
        }
        Messages::ClearEverything => {
            state.current_number = 0;
            state.input_number = 0;
        }
        Messages::Delete => {}
        Messages::Percentage => {}
    }
}

fn view(state: &AppState) -> Element<Messages> {
    column![
        column![
            text(state.input_number)
                .width(Length::Fill)
                .size(50)
                .align_x(Alignment::End),
            text(state.current_number)
                .width(Length::Fill)
                .size(50)
                .align_x(Alignment::End),
        ]
        .width(Length::Fill),
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
    .padding(10)
    .into()
}

fn main() -> iced::Result {
    iced::application("Calculator", update, view)
        .theme(|_s| iced::Theme::KanagawaDragon)
        .run()
}
