use iced::{
    widget::{button, column, container, row, text, text_input},
    Element,
};

use crate::{race::Race, utils};

#[derive(Debug, Clone, Default)]
pub struct CustomRace(pub(crate) Race);

impl CustomRace {
    pub fn new(race: Race) -> Self {
        Self(race)
    }

    pub fn view(&self) -> Element<Message> {
        let input = text_input("Enter name here...", &self.0.name).on_input(Message::NameEntered);

        container(
            column![
                row![
                    container(text("Race Name: ")).padding(utils::styles::row_adjusted_padding()),
                    input
                ]
                .spacing(2),
                button("Create").on_press(Message::CreateButtonPressed)
            ]
            .spacing(5),
        )
        .padding(styles::BASE_PADDING)
        .into()
    }

    pub fn update(&mut self, message: Message) -> Command {
        match message {
            Message::None => Command::None,

            Message::NameEntered(name) => {
                self.0.name = name.clone();
                Command::UpdatedCustomRace(self.clone())
            }

            Message::CreateButtonPressed => Command::CustomRaceCreated(self.0.clone()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    None,
    NameEntered(String),
    CreateButtonPressed,
}

#[derive(Debug, Clone)]
pub enum Command {
    None,
    UpdatedCustomRace(CustomRace),
    CustomRaceCreated(Race),
}

mod styles {
    use iced::Padding;

    use crate::utils;

    pub const BASE_PADDING: Padding = Padding {
        top: 15.0,
        bottom: 15.0,
        ..utils::styles::BASE_PADDING
    };
}
