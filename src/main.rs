use iced::{
    widget::{column, container},
    Element, Task,
};

use dregg::{
    frontend::{self, load_char_page, main_menu_btn, main_page, new_char_page},
    Message, Screen, State,
};

fn main() -> iced::Result {
    iced::run("Dregg", update, view)
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Main(msg) => {
            state.screen = Screen::Main(main_page::State {});
            let action = frontend::main_page::update(msg);
            match action {
                frontend::main_page::Action::None => Task::none(),
            }
        }
        Message::LoadCharacter(msg) => {
            state.screen = Screen::LoadCharacter(load_char_page::State {});
            let action = frontend::load_char_page::update(msg);
            match action {
                frontend::load_char_page::Action::None => Task::none(),
            }
        }
        Message::NewCharacter(msg) => {
            state.screen = Screen::NewCharacter(new_char_page::State::new());
            match &state.screen {
                Screen::NewCharacter(state) => {
                    let action = new_char_page::update(state, msg);
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
        Screen::Main(state) => {
            frontend::main_page::view(state);
            container(main_menu_btn().map(Message::Main)).into()
        }
        Screen::LoadCharacter(state) => {
            frontend::load_char_page::view(state);
            container(main_menu_btn().map(Message::Main)).into()
        }
        Screen::NewCharacter(state) => container(column![
            new_char_page::view(state).map(Message::NewCharacter),
            main_menu_btn().map(Message::Main)
        ])
        .into(),
    }
}
