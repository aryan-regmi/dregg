use iced::{Element, Task};

use dregg::{frontend, Message, Screen, State};

fn main() -> iced::Result {
    iced::run("Dregg", update, view)
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Main => {
            state.screen = Screen::Main;
            frontend::main_page::update(message)
        }
        Message::LoadCharacter => {
            state.screen = Screen::LoadCharacter;
            frontend::load_char_page::update(message)
        }
        Message::NewCharacter => {
            let new_char_page = frontend::new_char_page::NewCharacterPage::new();
            state.screen = Screen::NewCharacter(new_char_page);
            match &state.screen {
                Screen::NewCharacter(page) => page.update(message),
                _ => unreachable!(),
            }
        }
    }
}

fn view(state: &State) -> Element<Message> {
    match &state.screen {
        Screen::Main => frontend::main_page::view(state),
        Screen::LoadCharacter => frontend::load_char_page::view(state),
        Screen::NewCharacter(new_char_page) => new_char_page.view(),
    }
}
