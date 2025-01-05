use iced::{
    widget::{button, column},
    Element, Task,
};

fn main() -> iced::Result {
    iced::run("Dregg", update, view)
}

#[derive(Default)]
struct State {
    screen: Screen,
}

#[derive(Default)]
enum Screen {
    #[default]
    Main,
    LoadCharacter,
    NewCharacter,
}

#[derive(Debug, Clone)]
enum Message {
    Main,
    LoadCharacter,
    NewCharacter,
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Main => state.screen = Screen::Main,
        Message::LoadCharacter => state.screen = Screen::LoadCharacter,
        Message::NewCharacter => state.screen = Screen::NewCharacter,
    }

    Task::none()
}

fn view(state: &State) -> Element<Message> {
    match state.screen {
        Screen::Main => column![
            button("Load Character").on_press(Message::LoadCharacter),
            button("New Character").on_press(Message::NewCharacter),
        ]
        .into(),
        Screen::LoadCharacter => button("Main Menu").on_press(Message::Main).into(),
        Screen::NewCharacter => button("Main Menu").on_press(Message::Main).into(),
    }
}
