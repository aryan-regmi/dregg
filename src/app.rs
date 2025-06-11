use iced::{Element, Task};

use crate::{
    page::{self, Page},
    pages::new_character_page,
    race::{Race, Subrace},
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
        }
    }

    /// Renders the application.
    pub fn view(&self) -> Element<Message> {
        self.current_page.view()
    }
}

/// Props to pass to the `NewCharacterPage`.
#[derive(Default, Clone)]
pub struct NewCharacterPageProps {
    pub selected_race: Option<Race>,
    pub selected_subrace: Option<Subrace>,
}

/// The messages sent by the app.
#[derive(Debug, Clone)]
pub enum Message {
    MainMenuButtonPressed,
    NewCharacterButtonPressed(new_character_page::Message),
    LoadCharacterButtonPressed,
}
