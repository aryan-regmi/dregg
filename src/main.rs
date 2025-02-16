use iced::{Element, Task};

use dregg::{frontend, Message, Screen, State};

fn main() -> iced::Result {
    iced::run("Dregg", update, view)
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Main => frontend::main_page::update(state, message),
        Message::LoadCharacter => frontend::load_char_page::update(state),
        Message::NewCharacter => frontend::new_char_page::update(state),
    }
}

fn view(state: &State) -> Element<Message> {
    match state.screen {
        Screen::Main => frontend::main_page::view(state),
        Screen::LoadCharacter => frontend::load_char_page::view(state),
        Screen::NewCharacter => frontend::new_char_page::view(state),
    }
}
