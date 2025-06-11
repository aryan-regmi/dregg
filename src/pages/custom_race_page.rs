use iced::{
    widget::{button, column, container, row, text, text_input},
    Element,
};

use crate::race::Race;

#[derive(Debug, Clone, Default)]
pub struct CustomRace(pub(crate) Race);

impl CustomRace {
    pub fn new(race: Race) -> Self {
        Self(race)
    }

    pub fn view(&self) -> Element<Message> {
        // TODO: Add fields to create race, then add the created race to `self.availabe_races`

        let input = text_input("", &self.0.name).on_input(Message::NameEntered);

        container(column![
            row![text("Enter Name: "), input],
            button("Create").on_press(Message::CreateButtonPressed)
        ])
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
