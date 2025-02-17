use iced::{
    widget::{button, column, container},
    Element, Length, Task,
};

use dregg::{
    frontend::{self, load_char_page, main_menu_btn, new_char_page},
    Message, Screen, State,
};

fn main() -> iced::Result {
    iced::run("Dregg", update, view)
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Main => {
            state.screen = Screen::Main;
            Task::none()
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
            match &mut state.screen {
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
        Screen::Main => container(
            column![
                button("Load Character")
                    .on_press(Message::LoadCharacter(load_char_page::Message::None)),
                button("New Character")
                    .on_press(Message::NewCharacter(new_char_page::Message::None)),
            ]
            .spacing(20)
            .padding(20),
        )
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into(),
        Screen::LoadCharacter(state) => {
            frontend::load_char_page::view(state);
            main_menu_btn()
        }
        Screen::NewCharacter(state) => container(column![
            new_char_page::view(state).map(Message::NewCharacter),
            main_menu_btn()
        ])
        .into(),
    }
}
