use iced::button;
use iced::scrollable;
use iced::slider;
use iced::text_input;
use iced::{
    Alignment, Button, Checkbox, Column, Container, Element, Length,
    ProgressBar, Radio, Row, Rule, Sandbox, Scrollable, Settings, Slider,
    Space, Text, TextInput, Toggler,
};

use crate::Wordle::PositionInfo;

//struct InputGuess {
//    val: String,
//    block_colors: Vec<PositionInfo> // state to be updated
//}

enum GameInterface {
    Loading,
    Loaded//(GameState)
}

struct GameState {
    input: text_input::State,
    input_value: String,
    submit_btn: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    GuessSubmitted(String, u8),
    OutOfGuesses,
    BadGuess(String)

}

impl Sandbox for GameState {
    type Message = Message;

    fn new() -> Self {
        todo!()
    }

    fn title(&self) -> String {
        todo!()
    }

    fn update(&mut self, message: Self::Message) {
        todo!()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        todo!()
    }
}