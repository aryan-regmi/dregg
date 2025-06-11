use iced::{
    alignment::Horizontal,
    widget::{button, column, container},
    Border, Color, Element, Length,
};

use crate::{
    app::{Message, NewCharacterPageProps},
    pages::{
        custom_race_page::CustomRace,
        new_character_page::{self, NewCharacterPage},
    },
    race::{Race, Subrace},
};

/// Represents the various pages of the application.
#[derive(Default, Clone)]
pub enum Pages {
    #[default]
    Main,
    NewCharacter(NewCharacterPage),
    LoadCharacter,
}

#[derive(Default)]
pub struct Page {
    pub current: Pages,
    pub new_character_page_props: NewCharacterPageProps,
}

impl Page {
    pub fn new(current: Pages, new_character_page_props: NewCharacterPageProps) -> Self {
        Self {
            current,
            new_character_page_props,
        }
    }

    pub fn update(&mut self, message: Message) -> Command {
        match message {
            Message::MainMenuButtonPressed => Command::ChangePage(Pages::Main),

            Message::NewCharacterButtonPressed(msg) => {
                let mut new_character_page = NewCharacterPage::new(
                    self.new_character_page_props.selected_race.clone(),
                    self.new_character_page_props.selected_subrace.clone(),
                    self.new_character_page_props.available_races.clone(),
                    self.new_character_page_props.custom_race.clone(),
                );

                let command = new_character_page.update(msg);
                match command {
                    new_character_page::Command::None => {
                        Command::ChangePage(Pages::NewCharacter(new_character_page))
                    }
                    new_character_page::Command::RaceSelected(race) => {
                        self.new_character_page_props.selected_race = Some(race.clone());
                        self.current = Pages::NewCharacter(new_character_page);
                        Command::UpdateSelectedRace(race)
                    }
                    new_character_page::Command::SubraceSelected(subrace) => {
                        self.new_character_page_props.selected_subrace = Some(subrace.clone());
                        self.current = Pages::NewCharacter(new_character_page);
                        Command::UpdateSelectedSubrace(subrace)
                    }
                    new_character_page::Command::CustomRaceUpdated(race) => {
                        self.new_character_page_props.custom_race = race.clone();
                        self.current = Pages::NewCharacter(new_character_page);
                        Command::UpdateCustomRace(race)
                    }
                    new_character_page::Command::CustomRaceAdded(race) => {
                        self.new_character_page_props
                            .available_races
                            .push(race.clone());
                        self.current = Pages::NewCharacter(new_character_page);
                        Command::AddToAvailabeRaces(race)
                    }
                }
            }

            Message::LoadCharacterButtonPressed => Command::ChangePage(Pages::LoadCharacter),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let main_menu_btn = container(button("Main Menu").on_press(Message::MainMenuButtonPressed))
            .padding(20)
            .align_x(Horizontal::Center)
            .width(Length::Fill)
            .style(|_| container::Style {
                border: Border {
                    color: Color::from_rgb8(0, 0, 0),
                    width: 2.0,
                    ..Border::default()
                },
                ..Default::default()
            });

        match &self.current {
            Pages::Main => container(
                column![
                    Self::main_opts_button(
                        "New Character",
                        Message::NewCharacterButtonPressed(
                            new_character_page::Message::RaceButtonPressed
                        )
                    ),
                    Self::main_opts_button("Load Character", Message::LoadCharacterButtonPressed)
                ]
                .spacing(20),
            )
            .center(Length::Fill)
            .into(),

            Pages::NewCharacter(page) => container(column![
                page.view().map(Message::NewCharacterButtonPressed),
                main_menu_btn
            ])
            .into(),

            Pages::LoadCharacter => container(column![main_menu_btn]).into(),
        }
    }

    /// Creates a button in the main page.
    fn main_opts_button(name: &str, on_press: Message) -> Element<Message> {
        container(
            container(button(name).padding(10).on_press(on_press.clone())).center_x(Length::Fill),
        )
        .center_x(Length::Fill)
        .into()
    }
}

/// Represents commands a page can send to the application.
pub enum Command {
    ChangePage(Pages),
    UpdateSelectedRace(Race),
    UpdateSelectedSubrace(Subrace),
    UpdateCustomRace(CustomRace),
    AddToAvailabeRaces(Race),
}
