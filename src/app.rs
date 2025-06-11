use iced::{Element, Task};

use crate::{
    page::{self, Page},
    pages::{custom_race_page::CustomRace, new_character_page},
    race::{Race, Subrace},
    races,
};

/// The main application.
#[derive(Default)]
pub struct App {
    current_page: Page,

    new_character_page_props: NewCharacterPageProps,
}

impl App {
    /// Returns title of the app.
    pub fn title() -> &'static str {
        "Dregg"
    }

    /// Updates the application state.
    pub fn update(&mut self, message: Message) -> Task<Message> {
        let command = self.current_page.update(message);
        match command {
            page::Command::ChangePage(page) => {
                self.current_page = Page::new(page, self.new_character_page_props.clone());
                Task::none()
            }
            page::Command::UpdateSelectedRace(race) => {
                self.new_character_page_props.selected_race = Some(race);
                Task::none()
            }
            page::Command::UpdateSelectedSubrace(subrace) => {
                self.new_character_page_props.selected_subrace = Some(subrace);
                Task::none()
            }
            page::Command::UpdateCustomRace(custom_race) => {
                self.new_character_page_props.custom_race = custom_race;
                Task::none()
            }
            page::Command::AddToAvailabeRaces(race) => {
                self.new_character_page_props.available_races.push(race);
                Task::none()
            }
        }
    }

    /// Renders the application.
    pub fn view(&self) -> Element<Message> {
        self.current_page.view()
    }
}

/// Props to pass to the `NewCharacterPage`.
#[derive(Clone)]
pub struct NewCharacterPageProps {
    pub selected_race: Option<Race>,
    pub selected_subrace: Option<Subrace>,
    pub available_races: Vec<Race>,
    pub custom_race: CustomRace,
}

impl Default for NewCharacterPageProps {
    fn default() -> Self {
        Self {
            selected_race: Default::default(),
            selected_subrace: Default::default(),
            available_races: races::races(),
            custom_race: CustomRace::default(),
        }
    }
}

/// The messages sent by the app.
#[derive(Debug, Clone)]
pub enum Message {
    MainMenuButtonPressed,
    NewCharacterButtonPressed(new_character_page::Message),
    LoadCharacterButtonPressed,
}
