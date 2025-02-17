use iced::{
    widget::{button, column, container},
    Element, Length,
};

#[derive(Debug, Clone)]
pub enum Message {
    LoadCharacter,
    NewCharacter,
}

pub struct State {}

pub enum Action {
    None,
}

pub fn view(_: &State) -> Element<Message> {
    container(
        column![
            button("Load Character").on_press(Message::LoadCharacter),
            button("New Character").on_press(Message::NewCharacter),
        ]
        .spacing(20)
        .padding(20),
    )
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}

pub fn update(_: Message) -> Action {
    Action::None
}
