use iced::{
    widget::{column, container},
    Element, Task,
};

use dregg::{
    frontend::{self, main_menu_btn},
    Message, Screen, State,
};

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
        Message::NewCharacter(msg) => {
            let new_char_page = frontend::new_char_page::State::new();
            state.screen = Screen::NewCharacter(new_char_page);
            match &state.screen {
                Screen::NewCharacter(page) => {
                    let action = page.update(msg);
                    match action {
                        frontend::new_char_page::Action::None => Task::none(),
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

fn view(state: &State) -> Element<Message> {
    match &state.screen {
        Screen::Main => frontend::main_page::view(state),
        Screen::LoadCharacter => frontend::load_char_page::view(state),
        Screen::NewCharacter(new_char_page) => container(column![
            new_char_page.view().map(Message::NewCharacter),
            main_menu_btn()
        ])
        .into(),
    }
}
